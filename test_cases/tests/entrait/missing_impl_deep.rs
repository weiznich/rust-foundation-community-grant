struct Impl<T>(T);

trait T1 {
    fn t1(&self);
}

fn f1(deps: &impl T2) {
    deps.t2();
}

impl<T> T1 for Impl<T>
where
    Self: T2,
{
    fn t1(&self) {
        f1(self)
    }
}

trait T2 {
    fn t2(&self);
}

fn f2(deps: &impl T3) {
    deps.t3();
}

impl<T> T2 for Impl<T>
where
    Self: T3,
{
    fn t2(&self) {
        f2(self)
    }
}

trait T3 {
    fn t3(&self);
}

impl<T> T3 for Impl<T>
where
    Self: T4,
{
    fn t3(&self) {
        f3(self)
    }
}

fn f3(deps: &impl T4) {}

trait T4 {}

fn main() {
    let app = Impl(());
    // Note: The reason this fails is that T4 is not implemented for Impl<T>:
    app.t1();
}
