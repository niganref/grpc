## 使用技術


## 想定してるフォルダ構成
src/
├── modules/
│   ├── user/ #モジュール名         
│   │   ├── domain/   
│   │   ├── application/ 
│   │   ├── infrastructure/(dbのEntity周りどれがきれいか…モジュラーだからsharedの方がいいかもしれないORMとの連携が面倒)
│   │   │   ├──entity/？？
│   │   │   └──db/
│   │   │       └──repository.rs?
│   │   └── presentation/   
├── shared/ # 共有系
│   ├── domain/       
│   ├── infrastructure/
│   └── config.rs
└── main.rs   


## マイグレーションコマンド
sea-orm-cli migrate refresh