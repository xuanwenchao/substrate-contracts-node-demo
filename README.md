# 1. substrate 第五课作业
## 1.1 代码地址：
https://github.com/xuanwenchao/substrate-contracts-node-demo/tree/main/erc20

## 1.2 运行效果图
编译erc20合约工程
<img width="1066" alt="截屏2023-06-16 15 31 07" src="https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/19080102-2e7d-46eb-b69e-2996ed64f53f">
![2](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/93192952-ff9f-42a2-ac2a-bc6c375c67cf)
![3](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/6a422797-297d-43e3-af00-94690cccd0ed)
![4](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/056d7936-0a38-40a9-ab37-78db4509cfb6)
![5](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/e513b26a-115a-4805-941e-6872cf942819)
![6](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/6c2e23b0-4643-4b26-9297-1218939341c2)
![7](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/f3f0bdba-30b8-4e82-bf31-50c95ac97429)

## 1.3 测试用例执行结果
<img width="959" alt="截屏2023-06-19 11 52 56" src="https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/7d000381-d2e0-48e0-8bb8-86b55e7192ae">

# 2. Erc20+Erc721+Markets
## 2.1 上传合约文件
![截屏2023-06-18 14 51 19](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/6da0d52e-2bfd-489f-ad92-941cc1423450)

## 2.2 无法调用buyNft
<img width="1511" alt="截屏2023-06-18 21 26 11" src="https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/0b727ca2-10b5-4f17-b348-936dee9f2814">

## 2.3 approve授权
<img width="1529" alt="截屏2023-06-18 21 26 53" src="https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/871df5c3-2d93-47d2-8545-add16b0cbc2c">

## 2.4 成功调用buyNft
![截屏2023-06-18 14 51 41](https://github.com/xuanwenchao/substrate-contracts-node-demo/assets/1876277/d316872e-561b-4ae4-9f0b-ea903fca8c97)




# 3. 编译执行命令如下

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


做e2etest时，先指定contract-node节点的执行路径：
```js
export CONTRACTS_NODE=/Users/xuanwenchao/Documents/projects/Rust/substrate/substrate-contracts-node/target/release/substrate-contracts-node
```
然后执行测试命令
```js
cargo test --features e2e-tests
```
 
