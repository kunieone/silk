# Mixin in Silk

Silk 提倡使用 Mixin 实现组合式编程

## Example

```rust
ability Drivable{
  f drive():str
}

box Car {
  make:  str
  model: str,
  year:  UInt32
}

box Engine {
  horsepower:UInt32
  fuel_type:str
}

box CarWithEngine :: Drivable {
  car:Car = Car()
  engine:Engine = Engine()
  drive_history: str[] = []

  f drive():str {
    -> "Driving a {car.year} {car.make} {car.model}\
     with a {car.engine.horsepower} horsepower {car.engine.fuel_type} engine"
  }
}

box EngineCar mixin (car)Car,(engine)Engine {
  drive_history: str[] = []
  f drive():str {
    -> "Driving a {car.year}\
    {car.make} {car.model} with a {car.engine.horsepower}\
    horsepower {car.engine.fuel_type} engine"
  }
}

//equals to

box EngineCar  {
  @car: Car
  @engine: Engine
  @drive_history: str[] = []
  f drive():str {
    -> "Driving a {car.year}\
    {car.make} {car.model} with a {car.engine.horsepower}\
     horsepower {car.engine.fuel_type} engine"
  }
}
```
