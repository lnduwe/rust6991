use libssl_sys;

use libssl_sys::{SSL_library_init, SSL_load_error_strings};

fn main() {
    unsafe {
        // Initialize OpenSSL library
        SSL_library_init();
        SSL_load_error_strings();

        // Your code using OpenSSL functions goes here
        // For example, you can use SSL_CTX_new, SSL_new, etc.
    }

    // Rest of your program
}