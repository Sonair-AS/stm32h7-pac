How to set up the PAC


Clone the stm32-rs repo: https://github.com/stm32-rs/stm32-rs

Checkout version 0.15.1
 
Install correct versions of required tools:
 - cargo install svd2rust --version 0.24.1 --locked --force
 - cargo install svdtools --version 0.2.4 --locked


Follow instructions from the repo:
> * Unzip bundled SVD zip files: `cd svd; ./extract.sh; cd ..`
> * Generate patched SVD files: `make patch` (you probably want `-j` for all `make` invocations)
> * Generate svd2rust device crates: `make svd2rust`
> * Optional: Format device crates: `make form`

Copy the generated h7 pac files to this repo:
 `cp -r ./stm32h7/* <repo_parent>/stm32h7_pac`

