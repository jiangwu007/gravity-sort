use std::time::Instant;
use std::ptr;

/// AFT V12.7：第一次全局重心（求和/min-max），后续五点采样
struct AftCore;

impl AftCore {
    /// 公开接口：mode = 0 求和均值， mode = 1 min-max 均值
    pub fn sort_optimized(slice: &mut [i32], mode: u8) {
        let n = slice.len();
        if n <= 32 {
            slice.sort_unstable();
            return;
        }
        unsafe {
            Self::v12_7_unchained(slice.as_mut_ptr(), n, mode);
        }
    }

    /// 核心递归函数：phase 0 为第一次（全局重心），phase 1 为后续（五点采样）
    unsafe fn v12_7_unchained(mut ptr: *mut i32, mut n: usize, phase: u8) {
        while n > 32 {
            let alpha = if phase == 0 {
                // 第一次：使用全局重心
                if phase == 0 {
                    // 这里根据 mode 区分，但调用时 phase 与 mode 是同一个值，但为了清晰，我们按 mode 处理
                    // 由于 phase 传入的是 mode（0或1），我们可以直接使用 mode 的语义
                    // 为简化，我们直接在外部调用时决定：如果 phase==0 则使用求和均值；phase==1 使用 min-max 均值
                    // 但调用时 phase 就是 mode，所以我们可以区分
                    if phase == 0 {
                        // 求和均值
                        let mut sum: i64 = 0;
                        for i in 0..n {
                            sum += *ptr.add(i) as i64;
                        }
                        (sum / n as i64) as i32
                    } else {
                        // min-max 均值
                        let mut min = i32::MAX;
                        let mut max = i32::MIN;
                        for i in 0..n {
                            let val = *ptr.add(i);
                            if val < min { min = val; }
                            if val > max { max = val; }
                        }
                        ((min as i64 + max as i64) / 2) as i32
                    }
                } else {
                    unreachable!()
                }
            } else {
                // 后续递归：五点采样（首、1/4、中、3/4、尾的均值）
                let a = *ptr as i64;
                let b = *ptr.add(n / 4) as i64;
                let c = *ptr.add(n / 2) as i64;
                let d = *ptr.add(3 * n / 4) as i64;
                let e = *ptr.add(n - 1) as i64;
                ((a + b + c + d + e) / 5) as i32
            };

            // Hoare 分区（裸指针）
            let mut i = 0isize;
            let mut j = (n - 1) as isize;
            loop {
                while *ptr.offset(i) < alpha { i += 1; }
                while *ptr.offset(j) > alpha { j -= 1; }
                if i <= j {
                    ptr::swap(ptr.offset(i), ptr.offset(j));
                    i += 1;
                    j -= 1;
                }
                if i > j { break; }
            }

            let left_len = i as usize;
            let right_ptr = ptr.add(left_len);
            let right_len = n - left_len;

            // 尾递归优化：先处理较小的子数组，并继续循环处理较大的子数组
            // 注意：递归调用时，后续递归使用五点采样（phase=2）
            if left_len < right_len {
                Self::v12_7_unchained(ptr, left_len, 2);
                ptr = right_ptr;
                n = right_len;
            } else {
                Self::v12_7_unchained(right_ptr, right_len, 2);
                n = left_len;
            }
        }

        // 小数组收尾
        if n > 1 {
            std::slice::from_raw_parts_mut(ptr, n).sort_unstable();
        }
    }
}

fn main() {
    const N: usize = 10_000_000;
    println!("=== AFT V12.7 全局重心+后续五点采样 (两千万级) ===");

    // 生成随机数据
    let mut data: Vec<i32> = (0..N).map(|_| rand_fast()).collect();
    let mut data_std = data.clone();

    // 测试求和模式
    let mut data_sum = data.clone();
    let start = Instant::now();
    AftCore::sort_optimized(&mut data_sum, 0);
    let elapsed_sum = start.elapsed();
    println!("求和模式耗时: {:?}", elapsed_sum);

    // 测试 min-max 模式
    let mut data_minmax = data.clone();
    let start = Instant::now();
    AftCore::sort_optimized(&mut data_minmax, 1);
    let elapsed_minmax = start.elapsed();
    println!("min-max模式耗时: {:?}", elapsed_minmax);

    // 标准库 pdqsort
    let start = Instant::now();
    data_std.sort_unstable();
    let elapsed_std = start.elapsed();
    println!("pdqsort 耗时: {:?}", elapsed_std);

    // 验证正确性
    assert_eq!(data_sum, data_std);
    assert_eq!(data_minmax, data_std);
    println!("物理验证: ✅ 全部正确");

    println!("\n加速比 (求和模式 vs pdqsort): {:.2}x", elapsed_std.as_secs_f64() / elapsed_sum.as_secs_f64());
    println!("加速比 (min-max模式 vs pdqsort): {:.2}x", elapsed_std.as_secs_f64() / elapsed_minmax.as_secs_f64());
}

/// 快速随机数生成器（xorshift+）
static mut SEED: u64 = 88;
fn rand_fast() -> i32 {
    unsafe {
        SEED = SEED.wrapping_mul(6364136223846793005).wrapping_add(1);
        (SEED >> 33) as i32
    }
}



