echo "[launch openapi]"
echo "log file: ${LOG_PATH}/openapi.log"
sudo nohup ${ROOT_DIR}/bin/openapi ${ROOT_DIR}/config/openapi/config.toml > ${LOG_PATH}/openapi.log &