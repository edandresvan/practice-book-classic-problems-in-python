# README

This directory contains the This directory contains Python programs from each chapter with some modifications. Each program is implemented as a [Poetry](https://python-poetry.org/) project for managing dependencies and execution. Nevertheless, a single virtual environment is planned for the whole book.

# Requirements

I use Ubuntu Linux. So, if you want to run these programs, you may follow these minimal requirements or adapt them to your computer.

- Ubuntu Linux 22.04.
- [pyenv](https://github.com/pyenv/pyenv) for Python version management.

    - Install these packages before installing Python:
    
    ```shell
    sudo apt update; sudo apt install make build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm 
    libncursesw5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev
    ```

    - Install [pyenv-installer](https://github.com/pyenv/pyenv-installer) for an improved usage of pyenv: 
    
    ```shell
    curl https://pyenv.run | bash
    ```
    
    - Verify these lines were added to .bashrc and restart the terminal:
    
    ```shell
    export PATH="$HOME/.pyenv/bin:$PATH"
    eval "$(pyenv init --path)"
    eval "$(pyenv virtualenv-init -)"
    ```
    
    - Verify pyenv is working:
    
    ```shell
    $ pyenv --help
    ```
    
  - Install a Python version:
  
  ```shell
  $ pyenv install 3.11.1
  ```
   
  - Go to the root directory whre you want to save all the projects and create a Python virtual environment:
   
  ```shell
  $ pyenv local 3.11.1
  $ pyenv virtualenv classic-problems-pyenv
  ```
   
  - Install [Poetry](https://python-poetry.org/docs/#installation):
   
  ```shell
  $ curl -sSL https://install.python-poetry.org | python3 -;

  ```


  Add this your shell configuration file:

 ```shell

export PATH="/home/edandresvan/.local/bin:$PATH"

```


  - Go to a project directory.
  - Activate the Python environment:
  
  ```shell
   $ pyenv activate classic-problems-pyenv
  ```
  
  - Install project dependencies:
  
  ```shell
  $ poetry install
  ```
  
  - Run the project main function
  
  ```shell
  $ poetry run main
  ```
  
