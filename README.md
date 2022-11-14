<div align="center">
 <!-- #readme-top -->
 
<a name="readme-top"></a>
<h1 align="center">Webserver for Creating and Snapshotting VM</h1>
  <p align="center">
    HTTP Webserver for creating VMs and taking snapshots.
    <br />
    <!-- <a href="https://github.com/github_username/repo_name"><strong>Explore the docs »</strong></a> -->
    <!-- <br />
    <br />
    <a href="https://github.com/github_username/repo_name">View Demo</a>
    · -->
    <a href="https://github.com/chintansheth1711/col732_project_webserver/issues">Report Bug</a>
    
  </p>
</div>

<!-- [![crates.io](https://img.shields.io/crates/v/deduplication.svg)](https://crates.io/crates/deduplication) -->

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li> <a href="#rest-api-endpoints"> REST API Endpoint </a> 
        <ul>
            <li>
                <a href="#snapshot-post-api"> Snapshot Post API </a>
            </li>
            <li>
                <a href="#restore-and-create-vm-post-api"> Restore and Create VM Post API </a>
            </li>
        </ul>
        </li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- crates.io badge -->
- To reduce the overhead for the backend making use of our [vmm](https://github.com/anirudhakulkarni/732-demo.git) for creating new VMs and taking snapshots, webserver was designed to expose REST APIs.
- The VMM makes use of RPC to listen for actions such as taking snapshot and pause the VM from the external sources.

- The primary backend of the application cannot communicate with the VMM directly as it has been developed in Python and the RPC ties us to use the same language on client and server side.

- A central unit was required to manage the communication between the backend and the VMM which has been accomplished using Webserver implemented using [Rocket](https://rocket.rs/v0.5-rc/guide/introduction/) library.
- Webserver handles API requests for creating new VM's by forking new vmm-reference process with appropriate parameters.

<!-- REST API ENDPOINTS -->
### REST API Endpoints
<!-- SNAPSHOT POST API -->
#### Snapshot POST API 
<!-- DESCRIPTION -->
- **Description**

Snapshot API is used for creating snapshot of VM by forwarding snapshot request to appropriate VMM listening on given RPC port.
<!-- SNAPSHOT API ENDPOINT-->
- **Snapshot API Endpoint**
```
http://<Webserver IP>:<Webserver Port>/snapshot
```

<!-- PARAM -->
- **Parameters**
    - **cpu snapshot path (String)** : path at which cpu snapshot will be stored
    - **memory snapshot path (String)** : path at which memory snapshot will be stored
    - **rpc port (Integer)** : port corresponding to the VMM’s RPC server which is running the desired VM
    - **resume (Bool)** :
True if want to resume the VM after taking the snapshot.
False if want to halt the VM after taking the snapshot

- **Request Body**
```json
{
   "cpu_snapshot_path" : String,
   "memory_snapshot_path" : String,
   "rpc_port": Integer,
   "resume": Boolean
}
```
- **Response Body**
```json
{
    "Snapshot Taken Successfully"
}
```
<!-- RESTORE CREATE VM POST API -->
#### Restore and Create VM API Endpoint
- **Description**

Create and Restore API used for creating new VM either from kernel image or from saved cpu and memory snapshots.

- **Restore or Create API (POST) Endpoint**
```
http://<Webserver_IP>:<Webserver_Port>/create
```
- **Parameters**
    - **cpu snapshot path (String)** : the path to saved CPU snapshot if want to resume from existing VM
    - **memory snapshot path (String)** : the path to saved memory snapshot if want to resume from existing VM.
    - **kernel path (String)** : the path to base kernel image if want to create fresh VM from given kernel image.
    - **resume (Bool)** :
    true if want to resume from existing VM snapshot.
    false if want to create a fresh VM from base kernel image.
    
     
- **Request Body**
```json
{
   "cpu_snapshot_path" : String,
   "memory_snapshot_path" : String,
   "kernel_path" : String,
   "resume": Boolean
}
```
- **Response Body**
```json
{
   "pid" : Integer,
   "port" : Integer
}
```
<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

The server is developed entirely in rust. You need to install rust to use this library and packages mentioned in the `Cargo.toml` file.

### Prerequisites

Install rust from [here](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/chintansheth1711/col732_project_webserver.git
   ```
2. Install Rust packages
   ```sh
    cargo build
    ```
3. IP and Port at which webserver listens for API requests can be updated in **Rocket.toml** file
4. Starting Webserver
    ```sh
    cargo run
    ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage
1. Go to any API platform e.g. Postman and import the [API collection file](col732_project.postman_collection.json).
2. Update IP and Port of the webserver in the URL.
3. Update appropriate parameters from body of API requests.
4. Submit API request from postman.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See 
[`LICENSE`](
    LICENSE
) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>
 