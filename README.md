
# Image Compressor

## Overview

Image Compressor is a Rust-based application designed to efficiently reduce the file size of images while maintaining optimal quality. The project provides a fast, reliable, and user-friendly tool for compressing images, making it ideal for users who need to optimize image storage or improve loading times on websites and other platforms.

## Features

- **Efficient Image Compression**: Achieves up to 70% reduction in file size with minimal loss in image quality.
- **Batch Processing**: Supports batch processing, allowing users to compress multiple images simultaneously, saving time and improving workflow efficiency.
- **High Performance**: Utilizes Rust's performance and memory safety features to ensure fast and reliable image processing.
- **Cross-Platform**: Compatible with major operating systems, including Windows, macOS, and Linux.

## Technologies Used

- **Rust**: The core programming language used for the application, chosen for its performance, safety, and concurrency capabilities.
- **Image Processing**: Leveraged Rust libraries to handle image manipulation and compression tasks.

## Installation

To install and run the Image Compressor on your local machine, follow these steps:

### Prerequisites

- Ensure that you have Rust installed on your system. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Clone the Repository

```bash
git clone https://github.com/SamZhong2/Image_Compressor.git
cd Image_Compressor
```

### Build the Project

Compile the project using Cargo:

```bash
cargo build --release
```

### Run the Application

Once the project is built, you can run the application.

## Usage

### Compress an Image

To compress an image, use the following command:

```bash
./rpeg -c <PPM input filename> <Output filename>
```

- `-c`: Flag to initiate the compression process.
- `<PPM input filename>`: The path to the input image file in PPM format that you want to compress.
- `<Output filename>`: The desired output filename for the compressed image.

**Example:**

```bash
./rpeg -c input/image.ppm output/image_compressed.rpeg
```

This command compresses `image.ppm` and saves the compressed image as `image_compressed.rpeg`.

### Decompress an Image

To decompress an image, use the following command:

```bash
./rpeg -d <rpeg compressed filename> <Output filename>
```

- `-d`: Flag to initiate the decompression process.
- `<rpeg compressed filename>`: The path to the compressed image file in `.rpeg` format.
- `<Output filename>`: The desired output filename for the decompressed image.

**Example:**

```bash
./rpeg -d output/image_compressed.rpeg output/image_decompressed.ppm
```

This command decompresses `image_compressed.rpeg` and saves the decompressed image as `image_decompressed.ppm`.

