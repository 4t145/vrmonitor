LOG_PATH="/var/log/vrmonitor"
ROOT_DIR=$PWD
readonly LOG_PATH
readonly ROOT_DIR

function check_dir()
{
    if [ ! -d $1 ]
    then mkdir $1
    fi 
}

check_dir ${LOG_PATH};
check_dir ${ROOT_DIR}/bin;

# build server
cd ${ROOT_DIR}/server/openapi/
echo "[build server]"
cargo build --release
cd ${ROOT_DIR}
cp ${ROOT_DIR}/server/openapi/target/release/openapi ${ROOT_DIR}/bin

