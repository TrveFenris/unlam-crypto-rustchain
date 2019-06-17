
set server_manifest_location=server/Cargo.toml
set client_location=client
set ports=1337,1338,1339

echo {"ports":[%ports%]} > %client_location%/rustchain_ports.json
 
for %%a in ("%ports:,=" "%") do (
   start cmd /k cargo run --manifest-path %server_manifest_location%  -- %%~a
)

start /B cmd mkdir %client_location%\node_modules & cd %client_location% && npm install && npm start
