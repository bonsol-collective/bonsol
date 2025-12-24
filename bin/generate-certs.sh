#!/usr/bin/env bash
set -euo pipefail

# Create certs directory if it doesn't exist
mkdir -p certs
cd certs

CA_KEY="ca-key.pem"
CA_CERT="ca-cert.pem"
SERVER_KEY="server-key.pem"
SERVER_CSR="server.csr"
SERVER_CERT="server-cert.pem"
SERVER_EXT="server-ext.cnf"

echo "[1/4] Generating CA key and certificate..."
# CA private key
openssl genrsa -out "${CA_KEY}" 4096

# CA self-signed certificate
openssl req -x509 -new -nodes \
  -key "${CA_KEY}" \
  -sha256 -days 3650 \
  -subj "/CN=Bonsol Local CA" \
  -out "${CA_CERT}"

echo "[2/4] Generating server key..."
# Server private key
openssl genrsa -out "${SERVER_KEY}" 2048

echo "[3/4] Generating server CSR..."
# Server CSR
openssl req -new \
  -key "${SERVER_KEY}" \
  -subj "/CN=127.0.0.1" \
  -out "${SERVER_CSR}"

echo "[4/4] Creating server certificate with subjectAltName..."
cat > "${SERVER_EXT}" <<EOF
basicConstraints = CA:FALSE
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names

[alt_names]
IP.1 = 127.0.0.1
DNS.1 = localhost
EOF

# Sign server cert with our CA and include SAN
openssl x509 -req \
  -in "${SERVER_CSR}" \
  -CA "${CA_CERT}" \
  -CAkey "${CA_KEY}" \
  -CAcreateserial \
  -out "${SERVER_CERT}" \
  -days 3650 -sha256 \
  -extfile "${SERVER_EXT}"

rm -f "${SERVER_CSR}" "${SERVER_EXT}"

echo
echo "Done. Certificates generated in $(pwd)"