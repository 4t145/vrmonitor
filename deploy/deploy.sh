LOG_PATH="/var/log/vrmonitor"
ROOT_DIR=$1
readonly LOG_PATH
readonly ROOT_DIR


mkdir ${LOG_PATH}

# build server
cd ${ROOT_DIR}/server/openapi/
echo '[build server]'
cargo build --release

mkdir ${ROOT_DIR}/bin
cp ${ROOT_DIR}/server/openapi/target/openapi ${ROOT_DIR}/bin

echo '[launch openapi]'
nohup ${ROOT_DIR}/bin/openapi ${ROOT_DIR}/config/openapi/config.toml > ${LOG_PATH}/openapi.log &