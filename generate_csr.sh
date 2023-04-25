# Generate ecc key
openssl ecparam -name brainpoolP256r1 -genkey -out private.key
# Generate csr
openssl req -new -key private.key -out csr.csr -subj "/CN=$1/emailAddress=$2"
