# KAIVM - Shinkai Version Manager

KAIVM is a multiplatform Command Line Interface (CLI) designed to simplify the process of downloading, managing, configuring, and running different versions of Shinkai Node. It provides a seamless way to switch between Shinkai Node versions, ensuring developers can test their applications against multiple versions with ease. For more information on getting started with Shinkai Node, visit [Shinkai Documentation](https://docs.shinkai.com/getting-started).

Getting started with KAIVM is straightforward. Follow these steps to install, run, and customize a specific version of Shinkai Node:

## Getting Started

### 1. Install KAIVM

For macOS Apple Silicon users, the installation process involves a few simple steps. Please follow the instructions below:

  ```sh
  curl -L https://raw.githubusercontent.com/agallardol/kaivm/main/temp-release/kaivm-aarch64-apple-darwin -o kaivm
  chmod +x kaivm
  mv kaivm /usr/local/bin/
  ```

For users on other platforms, installation instructions are coming soon.

### 2. Install and run Shinkai Node

1. **Install a Shinkai Node Version**

   To install a specific version of Shinkai Node, use the `install` command followed by the version number. For example, to install version 0.5.3, you would run:

   ```
   kaivm install v0.5.3
   ```

2. **Run Shinkai Node**

   After installing the desired version, you can run it using the `shinkai-node run` command. This will start the Shinkai Node with the currently set version:

   ```
   kaivm shinkai-node run
   ```

3. **Customize Shinkai Node**

   You can customize your Shinkai Node instance by passing environment variables using the `--var=value` syntax. For example, to set the API port and the storage path, you can use:

   ```
   kaivm shinkai-node env --node_api_port=9550 --node_storage_path=~/.kaivm/data
   ```

By following these steps, you can easily manage, switch between different versions, and customize Shinkai Node for your development needs.


## Configuration and Assets

KAIVM stores its configuration files and downloaded Shinkai Node versions in the `~/.kaivm` directory. This centralized storage makes it easy to manage and access all KAIVM-related assets.

## Available Commands

KAIVM offers a variety of commands to manage Shinkai Node versions:

- **list**: Lists all installed Shinkai Node versions.
  ```
  kaivm list
  ```

- **install <VERSION>**: Downloads and installs a specific version of Shinkai Node.
  ```
  kaivm install 1.2.3
  ```

- **use <VERSION>**: Sets a specific version of Shinkai Node as the current version.
  ```
  kaivm use 1.2.3
  ```

- **version**: Displays the current Shinkai Node version.
  ```
  kaivm version
  ```

- **shinkai-node run**: Runs the currently set version of Shinkai Node.
  ```
  kaivm shinkai-node run
  ```

- **shinkai-node env**: Sets environment variables for the current Shinkai Node session. Available options include:
  - `--node_api_port`: Specifies the port on which the Shinkai Node API will run.
  - `--node_storage_path`: Defines the file system path where Shinkai Node will store its data.
  - `--unstructured_server_url`: The URL of the unstructured server that Shinkai Node will communicate with.
  - `--embeddings_server_url`: The URL of the embeddings server used by Shinkai Node for processing data.
  - `--first_device_needs_registration_code`: Determines whether the first device connecting to the Shinkai Node requires a registration code for authentication. Accepts 'true' or 'false'.
  - `--initial_agent_names`: A comma-separated list of initial agent names to be registered with the Shinkai Node.
  - `--initial_agent_urls`: A comma-separated list of URLs for the initial agents, corresponding to the names provided.
  - `--initial_agent_models`: A comma-separated list of models for the initial agents, corresponding to the names provided.
  - `--initial_agent_api_keys`: A comma-separated list of API keys for the initial agents, corresponding to the names provided.
  - `--starting_num_qr_devices`: The initial number of QR devices that should be supported by the Shinkai Node.
  ```
  kaivm shinkai-node env --node_api_port 9550 --node_storage_path ~/.kaivm/data
  ```

With KAIVM, managing Shinkai Node versions becomes a breeze, allowing developers to focus on building and testing their applications.
