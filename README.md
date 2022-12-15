# Fall 2022: EECS 589 Project
## Charles B Ziegenbein Jr, Ruohua Li, and Kevin Zhang

# System Set-Up
Our group utilized the Teaclave SDK with Docker on remote SGX-enabled Azure virtual machine.

In the virtual machine (ubuntu 18.04, 1vCPU, 4GB RAM), we have done the following
Installed SGX support libraries, see https://teaclave.apache.org/docs/my-first-function/ 
Installed docker and docker-compose
Pulled docker image baiduxlab/sgx-rust:1804-1.1.3

How to use?

SSH. Note the machine may be shut down between 2AM - 10 to save money. Username and sudo password are your umich unique names.
```
ssh -i your_key username@20.168.210.255
```

In your own directory, clone teaclave sdk. For instance, at /home/${USER}.
```
git clone https://github.com/apache/incubator-teaclave-sgx-sdk.git
```

Place your repo under incubator-teaclave-sgx-sdk/samplecode.
```
cd incubator-teaclave-sgx-sdk/samplecode && git clone your_repo && cd -
```

Launch docker container.
```
docker run -v /home/${USER}/incubator-teaclave-sgx-sdk:/root/sgx -ti --device /dev/sgx/enclave --device /dev/sgx/provision --name ${USER}_sgx baiduxlab/sgx-rust
```
-v mounts your directory into docker container. --name gives the container a unique name. -it brings you to interactive bash console.

Run your code.
(In container)
cd sgx/samplecode/your_repo
make # or whatever operation you want

Command exit will quit the container and the container will be closed. To restart:

```
docker start -ai ${USER}_sgx
```

# Our Repository
## Branches
We have numerous branches that are established to contain different portions of our system.

* SendingData -> used to implement the data sending portion of our protocol
* pcd_algorithm -> used to implement the receiver's PCD, also contains the pre-processing and hash/signature checks.
* python-prototype -> implementation of our python code for the PCD algorithm
* python_receiver -> implementation of Python socket programming to allow for data transfer.

# How to Run
Clone the repository onto an Azure machine that has the Baidu Rust SGX SDK docker container available. From here, establish two docker containers, as one will contain the 
sender and one will contain the receiver. To find the IP addresses for the docker containers, please use docker ps -a and get the container ID. From there, search for the IP
address in the details of that container.

For each folder, you need to run a ```make``` command to build the necessary files. After this completes successfully, navigate to the bin directory of each, and run the receiver's ./app executable first, then the sender.
