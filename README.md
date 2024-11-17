# IC File Storage Canister

A decentralized file storage system implemented as a canister smart contract on the Internet Computer blockchain. This system provides secure, efficient, and versioned file storage with metadata management and analytics capabilities.

## Features

- *File Management*
  - Upload and download files
  - Chunked file storage for efficient handling of large files
  - Version control for file modifications
  - File metadata management
  - Tag-based file organization and search

- *Storage Optimization*
  - Automatic file chunking (1MB chunks)
  - Maximum file size: 10MB
  - Total storage capacity: 1GB
  - Storage usage analytics

- *Metadata Support*
  - File size tracking
  - Upload and modification timestamps
  - File type categorization
  - Version history
  - Custom tags
  - Encryption status

## Technical Specifications

### Storage Limits
- Maximum file size: 10MB
- Chunk size: 1MB
- Total storage capacity: 1GB

### Data Structures
- FileMetadata: Stores comprehensive file information
- File: Contains file content and metadata
- FileStorage: Manages the overall storage system
- StorageError: Custom error handling

## API Reference

### Update Methods

#### upload_file
rust
upload_file(name: String, content: Vec<u8>, file_type: String, tags: Vec<String>) -> Result<bool, StorageError>

Uploads a new file with metadata and tags.

#### delete_file
rust
delete_file(name: String) -> Result<bool, StorageError>

Removes a file and its chunks from storage.

#### update_file_metadata
rust
update_file_metadata(name: String, new_tags: Option<Vec<String>>) -> Result<bool, StorageError>

Updates file metadata including tags.

#### create_file_version
rust
create_file_version(name: String, content: Vec<u8>) -> Result<bool, StorageError>

Creates a new version of an existing file.

### Query Methods

#### download_file
rust
download_file(name: String) -> Result<File, StorageError>

Retrieves a file and its metadata.

#### search_by_tags
rust
search_by_tags(tags: Vec<String>) -> Vec<File>

Searches for files matching specified tags.

#### get_storage_analytics
rust
get_storage_analytics() -> (usize, usize, usize)

Returns current storage usage, maximum storage size, and total file count.

#### get_file_type_distribution
rust
get_file_type_distribution() -> HashMap<String, usize>

Provides distribution of files by type.

## Error Handling

The system includes comprehensive error handling through the StorageError enum:
- FileNotFound
- FileAlreadyExists 
- InvalidOperation
- StorageLimit
- InvalidFileType
- SystemError

## Installation

1. Ensure you have the IC SDK installed
2. Clone this repository
3. Deploy using:
bash
dfx deploy ic_storage_canister


## Usage Example

bash
# Upload a file
dfx canister call ic_storage_canister upload_file '(
  "example.txt",
  blob "Hello World",
  "text/plain",
  vec {"document", "example"}
)'

# Download a file
dfx canister call ic_storage_canister download_file '("example.txt")'

# Search files by tags
dfx canister call ic_storage_canister search_by_tags '(vec {"document"})'


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
