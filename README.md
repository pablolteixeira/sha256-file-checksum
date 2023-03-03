SHA-256 File Checksum Algorithm was developed to help in verifying the integrity of a file you've downloaded, 
checking that the copy of the downloaded file is identical to the original.

A single character change in a file produces a totally different Hash in output.

To use the algorithm, you need to download the folder and inside it, execute the command below:
```shell
git clone https://github.com/pablolteixeira/sha256-file-checksum.git
```
Change the directory:
```shell
cd sha256-file-checksum
```
Execute the program:
```shell
./target/release/checksum-sha256 <file here>
``
