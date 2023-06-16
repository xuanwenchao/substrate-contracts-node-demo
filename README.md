# substrate 第五课作业
编译erc20合约工程
<img width="1066" alt="截屏2023-06-16 15 31 07" src="https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/19080102-2e7d-46eb-b69e-2996ed64f53f">
![2](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/93192952-ff9f-42a2-ac2a-bc6c375c67cf)
![3](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/6a422797-297d-43e3-af00-94690cccd0ed)
![4](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/056d7936-0a38-40a9-ab37-78db4509cfb6)
![5](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/e513b26a-115a-4805-941e-6872cf942819)
![6](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/6c2e23b0-4643-4b26-9297-1218939341c2)
![7](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/f3f0bdba-30b8-4e82-bf31-50c95ac97429)

下载编译substrate-contracts-node工程：
```js
git clone https://github.com/paritytech/substrate-contracts-node.git
cargo build --release
./target/release/substrate-contracts-node --dev --tmp
```

环境安装并创建合约工程：
```js
 cargo install dylint-link
 cargo install cargo-contract
 cargo contract new erc20
 ```

打开调试前端
https://contracts-ui.substrate.io/
