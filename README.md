## pagination 

use this for all pagination input in resolvers 

``#[graphql(default)] paginate: Paginate``

example: 
```rust
pub async fn brands(&self , #[graphql(default)] paginate: Paginate);
```

## export a list 

use this to return a list of models
``Result<List<Game>>``

to use this first use need to specify a unique name for your generic ``
#[graphql(concrete(name = "%your model name%", params(%your model struct % )))]
`` like these :
```rust

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
#[graphql(concrete(name = "categories", params(Category)))]
#[graphql(concrete(name = "products", params(Product)))]
#[graphql(concrete(name = "games", params(Game)))]
#[graphql(concrete(name = "brands", params(Brand)))]
pub struct List<D>
    where
        D: Sync,
        D: Send,
        D: OutputType
{
    pub data: Vec<D>,
    pub meta_data: MetaData,
}
```
usage example: 
```rust
pub async fn games(&self, #[graphql(default)] paginate: Paginate) -> Result<List<Game>>
```


## authorization