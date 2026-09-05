<p align="center">Декларативная библиотека для описания моделей данных для fugalang</p>

---

## Оглавление
  * [Начать использовать](#начать-использовать)

## Начать использовать

``` rust
imp (
  "chzcake" -> chz
)
fn Main() {
  const pkg := chz.Model!("User",
    pub name: str
    pub age: u8
  )
}
```
