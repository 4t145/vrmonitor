LOG_PATH="/var/log/vrmonitor"
ROOT_DIR=$PWD
echo "[launch openapi]"
echo "log file: ${LOG_PATH}/openapi.log"
nohup ${ROOT_DIR}/bin/openapi ${ROOT_DIR}/config/openapi/config.toml > ${LOG_PATH}/openapi.log &