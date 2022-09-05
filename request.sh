# Every request is served by a seperate thread without having to wait for other requests to complete
curl http://localhost:7878/sleep & curl http://localhost:7878/sleep & \

for i in {1..10}
   do
      curl http://localhost:7878/$i
   done