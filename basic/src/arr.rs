use std::collections::HashSet;

#[test]
fn test_collection() {
    let arr = [1, 2, 3];
    println!("arr is {:?}", arr);

    let mut arr1 = vec![1, 2, 3];
    arr1.push(4);
    println!("arr1 is {:?}", arr1);

    for item in arr {
        println!("item is {}", item);
    }

    for (index, item) in arr.iter().enumerate() {
        println!("index is {}, item is {}", index, item);
    }

    for (index, item) in arr1.iter().enumerate() {
        println!("index is {}, item is {}", index, item);
    }

    for i in 0..4 {
        println!("i is {}", i);
    }

    for i in 0..=4 {
        println!("i is {}", i);
    }

    // let new_arr = arr.map(|item| item * 2).collect::<Vec<_>>();
    // println!("new_arr is {:?}", new_arr);

    let arr = vec![1, 2, 3, 4, 5]; // 假设 arr 是一个 Vec<i32>
    let new_arr: Vec<i32> = arr.into_iter().map(|item| item + 1).collect();
    println!("new_arr is {:?}", new_arr);

    // map
    let arr = [1, 2, 3, 4, 5];
    let new_arr: Vec<i32> = arr.into_iter().map(|item| item + 1).collect();
    println!("new_arr is {:?}", new_arr);

    // iter
    let new_arr2 = arr.iter();
    println!("new_arr2 is {:?}", new_arr2);

    println!("vec iter: {:?}", arr.iter());

    // filter
    let new_arr3 = arr
        .into_iter()
        .filter(|item| *item > 3)
        .collect::<Vec<i32>>();
    println!("new_arr3 is {:?}", new_arr3);
}

// 迭代	iter()、iter_mut()、into_iter()
// 转换	map()、filter()、filter_map()、flat_map()
// 查找	find()、position()、rposition()、contains()、any()、all()
// 聚合	sum()、product()、fold()、reduce()、count()
// 组合	zip()、enumerate()、chain()、cycle()、take()、skip()
// 排序	sort()、sort_by()、partition()
#[test]
fn test_iter() {
    let arr = [1, 2, 3, 4, 5];

    println!("arr is {:?}", arr);

    // item是引用
    for item in arr.iter() {
        println!("item is {}", item);
    }

    // item所有权转移消费，但是后面arr还是能打印
    for item in arr.into_iter() {
        println!("item is {}", item);
    }
    println!("arr now is {:?}", arr);

    // into_iter在vec中
    let v = vec![1, 2, 3];
    for item in v.into_iter() {
        println!("item is {}", item);
    }
    // 下面一行打印会报错
    // println!("v is {:?}", v);
    let v = vec![1, 2, 3];
    for item in v.iter() {
        println!("item is {}", item);
    }
    println!("可以继续使用v is {:?}", v);

    // iter_mut可变
    let mut arr = [1, 2, 3, 4, 5];
    for item in arr.iter_mut() {
        *item += 1;
        println!("item is {}", item);
    }
    println!("mut arr with iter_mut arr is {:?}", arr);
}

#[test]
fn test_transform() {
    let arr = [1, 2, 3, 4, 5];

    // map:对每个元素应用一个闭包并返回新的迭代器
    let map_arr: Vec<i32> = arr.iter().map(|item| item + 1).collect();

    // map() 不会改变原始数据
    println!("map_arr is {:?},origin arr is {:?}", map_arr, arr);

    println!("arr iter is {:?}", arr.iter());
    // .collect() 将 Iterator 转换回 Vec<i32>
    println!(
        "without collect value is {:?}",
        arr.iter().map(|item| item + 1)
    );

    // filter与map不同，得用into_iter，闭包里才是 &i32类型。。。
    let evens: Vec<i32> = arr.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", evens); // 输出: [2, 4]

    // filter_map = filter+map，边筛选边转换
    // 例如将数组里偶数乘以2返回
    let double_evens: Vec<i32> = arr
        .into_iter()
        .filter_map(|item| if item % 2 == 0 { Some(item * 2) } else { None })
        .collect();
    println!("double_evens is {:?}", double_evens);

    // flat_map:类似 map()，但会将结果展开（flatten），适用于嵌套结构（如 Vec<Vec<T>>)
    let words = vec!["hello world", "rust is great", "functional programming"];
    let flat_words: Vec<&str> = words
        .into_iter()
        .flat_map(|word| word.split_whitespace())
        .collect();
    println!("flat_words is {:?}", flat_words);
    // 将多维数组展开
    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let flat_nested: Vec<i32> = nested.into_iter().flat_map(|item| item).collect();
    println!("flat_nested is {:?}", flat_nested);
}

#[test]
fn test_collect() {
    // collect的几种用法
    // Vec<T>
    let nums = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = nums.iter().map(|num| num * num).collect();
    println!("squared is {:?}", squared);

    // String
    let chars = vec!['y', 'm', 'n', 'e'];
    let join_str: String = chars.iter().collect();
    println!("join_str is {:?}", join_str);

    // HashSet<T>
    let nums = vec![1, 2, 3, 4, 5];
    let hash_set: HashSet<i32> = nums.into_iter().collect();
    println!("hash_set is {:?}", hash_set);

    let hash_set = hash_set.into_iter().collect::<HashSet<i32>>();
    println!("hash_set is {:?}", hash_set);
}

// 查找	find()、position()、rposition()、contains()、any()、all()
#[test]
fn test_find() {
    // find 查找第一个
    let nums = vec![1, 2, 3, 4, 5];
    let first_even = nums.iter().find(|item| *item % 2 == 0);
    println!("first_even is {:?}", first_even);

    // position查找满足条件的第一个位置，类似js的findIndex
    let first_even_position = nums.iter().position(|item| *item % 2 == 0);
    println!("first_even_position is {:?}", first_even_position);

    // reverse position，反向查，对应条件的数组下标(即查找最后一个匹配元素的索引）
    let r_even_position = nums.iter().rposition(|item| *item % 2 == 0);
    println!("first_even_position is {:?}", r_even_position);

    // contains 类似js的includes
    println!("nums is contain 5 = {}", nums.contains(&5));
    println!("nums is contain 6 = {}", nums.contains(&6));

    // any ,类似js的some
    println!(
        "nums has some value bigger than 4, result is {}",
        nums.iter().any(|x| *x > 4)
    );

    // all，类似js的every
    println!(
        "nums every item is bigger than 0,result is {}",
        nums.iter().all(|x| *x > 0)
    );
}

// 聚合	sum()、product()、fold()、reduce()、count()
#[test]
fn test_polymerization() {
    let nums = vec![1, 2, 3, 4, 5];

    // 求和
    let sum: i32 = nums.iter().sum();
    println!("sum is {:?}", sum);

    // 求积
    let product: i32 = nums.iter().product();
    println!("product is {:?}", product);

    // 累加聚合，类似js的reduce
    let fold = nums.iter().fold(10, |prev, current| {
        println!("prev is {}, current is {}", prev, current);
        prev + current
    });
    println!("fold is {:?}", fold);

    // reduce 类似fold，但是没有初始值！？？？
    let sum = nums.iter().copied().reduce(|acc, x| acc + x);
    println!("reduce is {:?}", sum);

    println!(
        "count of nums is {},len of nums is {}",
        nums.iter().count(),
        nums.len()
    );
}

// 组合	zip()、enumerate()、chain()、cycle()、take()、skip()
#[test]
fn test_combination() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![85, 90, 78];

    // zip
    let zip_result: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("zip_result is {:?}", zip_result);
    for (name, score) in zip_result {
        println!("{} score is {}", name, score);
    }

    // enumerate,返回索引
    for (item, index) in names.iter().enumerate() {
        println!("item is {}, index is {}", item, index);
    }

    // chain 链接迭代器，类似js的concat
    let scores2 = vec![100, 68, 50];
    let all_scores: Vec<_> = scores.iter().chain(scores2.iter()).collect();
    println!("all_scores is {:?}", all_scores);

    // cycle 无限循环，直到take来停止
    let cycle_seven = scores.iter().cycle().take(7).collect::<Vec<_>>();
    println!("cycle_seven is {:?}", cycle_seven);

    // take 取前几个元素
    println!(
        "prev two items is {:?}",
        scores.iter().take(2).collect::<Vec<_>>()
    );

    // skip 跳过对应数量的元素
    let nums = vec![1, 2, 3, 4, 5];
    for item in nums.iter().skip(2).collect::<Vec<&i32>>() {
        println!("item is {}", item);
    }
}

// 排序	sort()、sort_by()、partition()
#[test]
fn test_sort() {
    let mut nums = vec![5, 3, 8, 1, 4];

    // sort无返回值，只在原数组上操作。升序排序
    println!("{:?}", nums.sort());
    println!("nums is {:?}", nums);

    // sort_by自定义排序规则
    let mut nums = vec![5, 3, 8, 1, 4];
    nums.sort_by(|a, b| b.cmp(a));
    println!("nums is {:?}", nums);

    let mut words = vec!["apple", "banana", "kiwi", "cherry"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("words is {:?}", words);

    // partition 按条件拆分为两个部分
    let words = vec!["apple", "banana", "kiwi", "cherry"];
    let partition_result: (Vec<&str>, Vec<&str>) = words.iter().partition(|item| item.len() < 5);
    println!(
        "partition_result is {:?}, left is match => {:?}",
        partition_result, partition_result.0
    );
}
