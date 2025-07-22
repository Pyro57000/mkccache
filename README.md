# mkccache
Take raw Rubeus output and makes a ccache file that you can use to proxy kerberos auth.

# Usage
Simply run mkccache and then copy the ticket information that rubeus spits out and paste it into the tool when prompted.

Once the output is pasted in type the line END and press enter.

It will automatically convert this to a usable format and save it as ccache file, it will then paste the command you need to set the environment variable, simply copy and paste that into your terminal!


# Installation
1. download the latest release binary
2. copy that binary somewhere on your path, or call it directly on your file system
3. that's it!

# Compile it
1. git clone this repo
2. cd mkccache
3. cargo build --release
4. copy the resulting binary in mkccache/target/release/mkccache to somewhere in your path.

# Dependencies
1. impacket's ticket converter python script installed on your path
2. that's it. 
