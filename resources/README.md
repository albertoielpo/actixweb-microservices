# SSL CERT
Place in this directory cert.pem and key.pem.

For test purposes you can generate your own self signed with openssl 

<code>openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'</code>