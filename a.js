class Bird {
  constructor(name) {
    this.name = name;
  }
}

let bird = new Bird("A");
console.log(bird);
let bird2 = bird;
bird2.name = "Good";
console.log(bird);
