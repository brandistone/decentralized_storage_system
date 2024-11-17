use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;
use std::cell::RefCell;

// Custom error type for better error handling
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum StorageError {
    FileNotFound,
    FileAlreadyExists,
    InvalidOperation,
    StorageLimit,
    InvalidFileType,
    SystemError,
}

// Enhanced file metadata
#[derive(CandidType, Clone, Deserialize)]
struct FileMetadata {
    name: String,
    size: usize,
    upload_timestamp: u64,
    last_modified: u64,
    file_type: String,
    is_encrypted: bool,
    version_history: Vec<String>, // For version control
    tags: Vec<String>,
}

#[derive(CandidType, Clone, Deserialize)]
struct File {
    name: String,
    content: Vec<u8>,
    metadata: FileMetadata,
}

#[derive(Default)]
struct FileStorage {
    files: HashMap<String, File>,
    file_chunks: HashMap<String, Vec<Vec<u8>>>, // For storing file chunks
    storage_usage: usize,
    max_storage_size: usize,
}

// Constants
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const CHUNK_SIZE: usize = 1024 * 1024; // 1MB per chunk
const MAX_STORAGE_SIZE: usize = 1024 * 1024 * 1024; // 1GB total storage

thread_local! {
    static STATE: RefCell<FileStorage> = RefCell::new(FileStorage {
        files: HashMap::new(),
        file_chunks: HashMap::new(),
        storage_usage: 0,
        max_storage_size: MAX_STORAGE_SIZE,
    });
}

// Helper function to get current time
fn get_current_timestamp() -> u64 {
    ic_cdk::api::time() / 1_000_000_000 // Convert nanoseconds to seconds
}

// Initialize the canister
#[init]
fn init() {
    // Initialization handled by thread_local
}

// CRUD Operations with error handling
#[update]
fn upload_file(name: String, content: Vec<u8>, file_type: String, tags: Vec<String>) -> Result<bool, StorageError> {
    STATE.with(|state| {
        let mut storage = state.borrow_mut();
        
        // Validate file size
        if content.len() > MAX_FILE_SIZE {
            return Err(StorageError::StorageLimit);
        }
        
        // Check storage capacity
        if storage.storage_usage + content.len() > storage.max_storage_size {
            return Err(StorageError::StorageLimit);
        }

        // Create file metadata
        let metadata = FileMetadata {
            name: name.clone(),
            size: content.len(),
            upload_timestamp: get_current_timestamp(),
            last_modified: get_current_timestamp(),
            file_type,
            is_encrypted: false,
            version_history: Vec::new(),
            tags,
        };

        // Split file into chunks for better management
        let chunks: Vec<Vec<u8>> = content
            .chunks(CHUNK_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect();

        let file = File {
            name: name.clone(),
            content: content.clone(),
            metadata,
        };

        storage.files.insert(name.clone(), file);
        storage.file_chunks.insert(name, chunks);
        storage.storage_usage += content.len();

        Ok(true)
    })
}

#[query]
fn download_file(name: String) -> Result<File, StorageError> {
    STATE.with(|state| {
        let storage = state.borrow();
        storage.files
            .get(&name)
            .cloned()
            .ok_or(StorageError::FileNotFound)
    })
}

#[update]
fn delete_file(name: String) -> Result<bool, StorageError> {
    STATE.with(|state| {
        let mut storage = state.borrow_mut();
        
        if let Some(file) = storage.files.remove(&name) {
            storage.storage_usage -= file.content.len();
            storage.file_chunks.remove(&name);
            Ok(true)
        } else {
            Err(StorageError::FileNotFound)
        }
    })
}

#[update]
fn update_file_metadata(
    name: String,
    new_tags: Option<Vec<String>>,
) -> Result<bool, StorageError> {
    STATE.with(|state| {
        let mut storage = state.borrow_mut();
        
        if let Some(file) = storage.files.get_mut(&name) {
            if let Some(tags) = new_tags {
                file.metadata.tags = tags;
            }
            file.metadata.last_modified = get_current_timestamp();
            Ok(true)
        } else {
            Err(StorageError::FileNotFound)
        }
    })
}

#[update]
fn create_file_version(name: String, content: Vec<u8>) -> Result<bool, StorageError> {
    STATE.with(|state| {
        let mut storage = state.borrow_mut();
        
        if let Some(file) = storage.files.get_mut(&name) {
            let version_id = format!("{}_{}", name, get_current_timestamp());
            file.metadata.version_history.push(version_id.clone());
            
            // Store the old version in chunks
            let chunks: Vec<Vec<u8>> = content
                .chunks(CHUNK_SIZE)
                .map(|chunk| chunk.to_vec())
                .collect();
                
            storage.file_chunks.insert(version_id, chunks);
            Ok(true)
        } else {
            Err(StorageError::FileNotFound)
        }
    })
}

#[query]
fn search_by_tags(tags: Vec<String>) -> Vec<File> {
    STATE.with(|state| {
        let storage = state.borrow();
        storage
            .files
            .values()
            .filter(|file| {
                tags.iter()
                    .all(|tag| file.metadata.tags.contains(tag))
            })
            .cloned()
            .collect()
    })
}

#[query]
fn get_storage_analytics() -> (usize, usize, usize) {
    STATE.with(|state| {
        let storage = state.borrow();
        (
            storage.storage_usage,
            storage.max_storage_size,
            storage.files.len()
        )
    })
}

#[query]
fn get_file_type_distribution() -> HashMap<String, usize> {
    STATE.with(|state| {
        let storage = state.borrow();
        let mut distribution = HashMap::new();
        
        for file in storage.files.values() {
            *distribution
                .entry(file.metadata.file_type.clone())
                .or_insert(0) += 1;
        }
        
        distribution
    })
}

// This is needed for candid interface generation
ic_cdk::export_candid!();