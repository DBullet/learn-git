##问题描述
一个局域网内部的两台计算机的A、B的子网应该是192.168.26.0/24，网关192.168.26.2，如下图：
![topo](https://github.com/DBullet/learn-git/blob/test/image001.png) 
其中B的子网掩码本应该是255.255.255.0，现在错误地配置成了255.255.255.224，请问A和B之间是否能正常通信。

##分析与结论：
在其他配置均正确的情况下，A和B之间仍然能够正常通信。分两种情况进行分析。
1. A向B发送数据，B接收来自A的数据
B的子网掩码错误并不影响A向B发送数据以及B接收来自A的数据，A向B发送数据时，首先判断B的IP地址与自身在同一个子网内，则在获取B的Mac地址后会直接将数据发送给B，而B也正常接收。
2. B向A发送数据，A接收来自B的数据
当B向A发送数据是，B通过自身的子网掩码判断目的地A的IP地址与自身是否在同一个子网内，由于B的子网掩码配置发生错误，B认为A与自身不在同一个子网内，因此，B会将发送给A的数据发送给网关，由网关进行路由选择，而网关会正确地把发给A的数据转发给A，因此A仍然能收到来自B的数据，不同的是，B误以为A与自身不在同一个子网，而将数据发送给网关，经由网关转发给A，而正常情况下，B会直接将数据发送给同一个子网内的A。

综合以上分析可以得出结论：A和B之间仍然能够正常通信。
##实验验证：
- 实验环境配置为：
在Win7系统下开启VMWare下的两台虚拟机，一台系统为Ubuntu 17.04，另一台系统为Kali 2017.02。网络配置均为NAT，即两台虚拟机以物理机为网关组成了一个局域网。
- 初始正常情况下，两台主机的IP地址和网关配置如下：
	- Kali： IP 192.168.30.131  gateway：192.168.30.2  Mask：255.255.255.0
	- Ubuntu： IP 192.168.30.128  gateway：192.168.30.2  Mask：255.255.255.0
![03](https://github.com/DBullet/learn-git/blob/test/image003.png)
![05](https://github.com/DBullet/learn-git/blob/test/image005.png)
#### 实验操作
1. 现在将Ubuntu系统主机的IP地址更改为192.168.30.3，子网掩码不变。更改后的配置如下：![07](https://github.com/DBullet/learn-git/blob/test/image007.png) 
此时，Kali和Ubuntu能够正常通信。
![09](https://github.com/DBullet/learn-git/blob/test/image009.png)
![11](https://github.com/DBullet/learn-git/blob/test/image011.png) 
2. 进一步，将Ubuntu的子网掩码改为255.255.255.224。更改后的配置如下：![13](https://github.com/DBullet/learn-git/blob/test/image013.png)此时发现，Ubuntu与Kali不能正常通信，Kali能够正常访问外网，而Ubuntu不能访问外网，但能够与同一个子网下的网关正常通信。
![15](https://github.com/DBullet/learn-git/blob/test/image015.png)
![17](https://github.com/DBullet/learn-git/blob/test/image017.png) 
从Ubuntu不能访问外网而能和同一个子网内的网关正常通信，说明Ubuntu的网关配置尚且不完善，确实，在更改了Ubuntu的掩码后，还未给Ubuntu配置网关，此时Ubuntu不能与Kali通信，也说明了Ubuntu并没有把Kali作为与自身在同一个子网下的主机。这与之前的分析是相符的。通过进一步的确认也能够验证Ubuntu网关未正确配置的推断的正确性。
![19](https://github.com/DBullet/learn-git/blob/test/image019.png)
4. 将Ubuntu的网关配置正确。
![21](https://github.com/DBullet/learn-git/blob/test/image021.png)
配置正确后能够看到Ubuntu和Kali又能够正常通信了，且Ubuntu也能正常访问外网了。
![23](https://github.com/DBullet/learn-git/blob/test/image023.png)
![25](https://github.com/DBullet/learn-git/blob/test/image025.png)
由网关配置正确后，Kali和Ubuntu才能正常通信可以验证之前的分析，即由Ubuntu发向Kali的数据需要经过网关的转发。