#  -*- coding: utf-8 -*-
import random
import time
import statistics
from typing import Union,List, Optional, Callable,Tuple


def gravity_sort(arr: List[float | int]) -> Union[List[int], List[float]]:
    '''重力排序'''
    alpha = int(sum(arr) / len(arr))
    left:List[int | float] = []
    right:List[int | float] = []
    for x in arr:
        if x < alpha:
            left.append(x)
        else:
            right.append(x)
    if not left or not right: return sorted(arr)
    return gravity_sort(left) + gravity_sort(right)


def yh_gravity_sort(arr: List[float | int]) -> Union[List[int], List[float]]:
    '''重力排序,纯python'''
    if len(arr) <= 1: return arr
    alpha:float = sum(arr) / len(arr)
    left:List[int | float] = []
    right:List[int | float] = []
    for x in arr:
        if x < alpha:
            left.append(x)
        else:
            right.append(x)
    if not left or not right: # 既然均值切不动，说明数据分布极度偏斜。
        pivot:int | float = arr[len(arr) // 2]
        return gravity_sort([x for x in arr if x <= pivot]) + gravity_sort([x for x in arr if x > pivot])
    return gravity_sort(left) + gravity_sort(right)


def quick_sort(arr: List[float | int]) -> Union[List[int], List[float]]:
    '''快速排序'''
    def median_three(a:int | float, b:int | float, c:int | float) -> Union[int, float]:
        '''采用三点取样'''
        if a < b:
            if b < c: return b
            elif a < c: return c
            else: return a
        else:
            if a < c: return a
            elif b < c: return c
            else: return b
    
    def qs(l:int, r:int) -> Optional[Union[List[int], List[float]]]:
        if l >= r: return None
        pivot:Union[int, float] = median_three(arr[l], arr[(l + r) // 2], arr[r])
        i, j = l, r
        while i <= j:
            while arr[i] < pivot:
                i += 1
            while arr[j] > pivot:
                j -= 1
            if i <= j:
                arr[i], arr[j] = arr[j], arr[i]
                i += 1
                j -= 1
        qs(l, j)
        qs(i, r)
    qs(0, len(arr) - 1)
    return arr


# ==================== 数据生成 ====================
def generate_data(size:int, dist:str) -> Union[List[int], List[float]]:
    if dist == 'uniform':
        return [random.random() for _ in range(size)]
    elif dist == 'normal':
        return [random.gauss(0, 1) for _ in range(size)]
    elif dist == 'bimodal':
        half:int = size // 2
        return [random.gauss(0, 1) for _ in range(half)] + [random.gauss(100, 10) for _ in range(size - half)]
    elif dist == 'duplicates':
        return [random.randint(0, 100) for _ in range(size)]
    else: # sorted,生成一个近似有序的数组
        arr:List[int] = list(range(size))
        for _ in range(size // 100):
            i, j  = random.sample(range(size), 2)
            arr[i], arr[j] = arr[j], arr[i]
        return arr


# ==================== 计时器 ====================
def benchmark(func:Callable, data:Union[List[int], List[float]], repeat:int = 3) -> Tuple[float, Union[List[int], List[float]]]:
    times:List[float] = []
    result:Union[List[int], List[float]] = []
    for _ in range(repeat):
        arr: Union[List[int], List[float]] = data.copy()
        start:float = time.perf_counter()
        result:Union[List[int], List[float]] = func(arr)
        times.append(time.perf_counter() - start)
    return statistics.mean(times), result


# ==================== 主测试 ====================
def run_benchmark():
    random.seed(9527)
    sizes:List[int] = [500000, 1000000, 2000000, 4000000, 6000000, 8000000]
    dists:List[str] = ["uniform","normal","bimodal","sorted","duplicates"]
    for size in sizes:
        print(f'规模 N = {size}')
        print('=' * 15)
        for dist in dists:
            data:Union[List[int], List[float]] = generate_data(size=size, dist=dist)
            random.shuffle(data)
            t_quick, r1 = benchmark(quick_sort, data=data)
            t_gravity, r2 =  benchmark(gravity_sort, data=data)
            aft_gravity, r3 = benchmark(yh_gravity_sort, data=data)
            print(f'\n数据分析: {dist}')
            print(f'Quick Sort: {t_quick:.4f}s')
            print(f'Gravity Sort: {t_gravity:.4f}s')
            print(f'Yunhai Gravity: {aft_gravity:.4f}s')
            print(f'Gravity Sort VS Quick Sort: {t_quick / t_gravity:.2f}x')
            print(f'Yunhai Gravity VS Quick Sort: {t_quick / aft_gravity:.2f}x')
            print("正确性:", "✅" if r1==r2==r3 else "❌")


if __name__ == "__main__":
    run_benchmark()