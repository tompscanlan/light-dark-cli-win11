FROM rust:latest 
 
RUN apt update &amp;&amp; apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64 
 
RUN rustup target add x86_64-pc-windows-gnu 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu 
 
WORKDIR /app 
 
CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu"] 


cargo build --target x86_64-pc-windows-gnu

cd /tmp
git clone https://github.com/tompscanlan/light-dark-cli-win11.git

 cd light-dark-cli-win11/