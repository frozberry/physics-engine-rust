# Returning Option<Contact> from Collision Checker

Returning an option very expressive way of describing a collision check function.

In C++ is_colliding needs to return boolean to test agains the conditional. So it has to initialise an empty Contact and mutate it in place.

```c++
bool CollisionDetection::IsColliding(Body* a, Body* b, Contact& contact)
```

```c++
Contact contact;
if (CollisionDetection::IsColliding(a, b, contact)) {
	ResolveContact(contact)
}
```

`Option<Contact>` is much more natural. The option can be destructured and conditionally run with `if let`.

```rust
pub fn is_colliding(a: &mut Body, b: &mut Body) -> Option<Contact>
```

```rust
let maybe_contact =	collision::is_colliding(&mut a, &mut b);
if let Some(contact) = maybe_contact {
	resolve_contact(contact)
}
```

# Using Enum instead of Inheritance

I enjoy using Enums to describe different variants more than the traditional OOP way. This is not a techincal judgement, it simply brings me joy to code this way.

```rust
pub enum Shape {
    Circle(f32),
    Polygon(Vec<Vec2>),
    Box(f32, f32),
}

struct Body {
	...
}
```

Shape specific can be moved out of body and implemented on the `Shape` enum.

```rust
impl Shape {
    pub fn calc_inertia(&self, mass: f32) -> f32 {
        match self {
            Shape::Circle(radius) => circle_inertia(radius, mass),
            Shape::Polygon(vertices) => polygon_inertia(vertices, mass),
            Shape::Box(w, h) => box_inertia(w, h, mass),
        }
	}
}
```

Rust's `match` is a pleasure to use. The compiler also enforces that all enum variants are considered.

A nested `match` for collision detection.

```rust
pub fn is_colliding(a: & Body, b: & Body) -> Option<Contact> {
    match a.shape {
        Shape::Circle(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_circle(a, b),
            Shape::Polygon(_) => is_collidng_circle_polygon(a, b),
            Shape::Box(_, _) => is_collidng_circle_box(a, b),
        },
        Shape::Polygon(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_polygon(a, b),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_box(a, b),
        },
        Shape::Box(_, _) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_box(a, b),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_box(a, b),
        },
    }
}
```

It can be a bit overly verbose when the compiler is not able to verify that only a certain variant will be passed in.

```rust
pub fn is_collidng_circle_circle(a: &Body, b: &Body) -> Option<Contact> {
    match a.shape {
        Shape::Circle(a_r) => match b.shape {
            Shape::Circle(b_r) => {
				// do stuff
            }
            _ => panic!("Non-circle passed into collision function"),
        },
        _ => panic!("Non-circle passed into collision function"),
    }
}
```

This `match` could be replaced with `if let Shape::Circle(r)`. But I like the robustness of writing the full match statements. This should catch bugs in development if we accidently have code that calls the function with the wrong arguments.

This has disavantages too though: 
```rust
fn get_local_verticies(&self) -> Vec<Vec2> {
    match self {
        Shape::Circle(_) => panic!("Circle has no verticies"),
        Shape::Polygon(vertices) => vertices.to_vec(),
        Shape::Box(w, h) => get_box_verticies(w, h)
    }
}
```
We have a function to get the verticies of a shape, but this only makes sense for `Box` and `Polygon`. A `Circle` has no verticies. So here we simply crash at runtime if the method is called on `Circle`. A traditional OOP approach would allow the compiler to ensure that we never call the method on a `Circle`.

A more idomatic way to do this would be to return an `Option<Vec<Vec2>>`, and simply return `None` for `Circle`s. But this does add some additional verbosity as we would have to handle the `Option` each time we called this method.  


# Borrow Checker Problems

For resolving collisions, I originally I wanted to:

```rust
for i in 0..self.bodies.len() {
	for j in (i + 1)..self.bodies.len() {
		if i != j {
			let maybe_contact =
				collision::is_colliding(&mut self.bodies[i], &mut self.bodies[j]);
		}
	}
}
```

```
`is_colliding()` needs mutable references to the bodies, since they will be used to instantiate a `Contact` struct, which needs to mutate the bodies position.

But the borrow checker does not like this.
```

error[E0499]: cannot borrow `self.bodies` as mutable more than once at a time

````

I think the problem is that the compiler does now know that `i != j`, so it thinks we could be accessing the same element twice. This would break the borrow checker rule of only a single mutable reference being allowed.

To work around this I used `split_at_mut()`.
``` rust
let mut v = [1, 0, 3, 0, 5, 6];
let (left, right) = v.split_at_mut(2);
// [1, 0] [3, 0, 5, 6]
````

Now the compiler knows we aren't accessing the same element twice.

```rust
for i in 0..self.bodies.len() {
	for j in (i + 1)..self.bodies.len() {
		if i != j {
			let (left, right) = self.bodies.split_at_mut(i + 1);
			let maybe_contact =
				collision::is_colliding(&mut left[i], &mut right[j - i - 1]);
		}
	}
}
```

This is not a very readable solution, and the indexing arithmatic is not obvious at first glance. I think another solution would be to use interior mutability like a `Cell` or `RefCell`, which I haven't looked into much.

As an aside: I was originally confused at the naming of `split_at_mut()`. What was the "mut" that was being "split_at"? Then I realised that it was just the mutable version of `split_at()`. I.e "split slice at n" and "split mutable slice at n".
