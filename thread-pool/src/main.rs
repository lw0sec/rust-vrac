use thread_pool::pool::ThreadPool;

type TaskResult<T> = std::result::Result<T, TaskError>;

struct TaskError;

fn task() -> TaskResult<i32> {
    Ok(42)
}

fn run(val: i32) {
    println!("{val}");
}

fn main() {
    let pool = ThreadPool::new(5);

    let val: i32 = 42;
    pool.execute(move || run(val));
}
