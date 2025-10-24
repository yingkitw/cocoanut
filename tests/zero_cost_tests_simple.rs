//! Simplified zero-cost abstractions tests - only testing actually implemented methods

use cocoanut::*;
use cocoanut::zero_cost::ZeroCostArray;

#[test]
fn test_zero_cost_array_creation() {
    // Test that we can create a zero-cost array
    let data = [1, 2, 3, 4, 5];
    let arr = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    assert_eq!(arr.len(), 5);
    assert!(!arr.is_empty());
}

#[test]
fn test_zero_cost_array_operations() {
    let data = [1, 2, 3, 4, 5];
    let arr = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    
    // Test length
    assert_eq!(arr.len(), 5);
    assert!(!arr.is_empty());
    
    // Test element access
    assert_eq!(arr.get(0), Some(&1));
    assert_eq!(arr.get(2), Some(&3));
    assert_eq!(arr.get(4), Some(&5));
    assert_eq!(arr.get(5), None);
}

#[test]
fn test_zero_cost_array_iteration() {
    let data = [1, 2, 3, 4, 5];
    let arr = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    
    // Test iteration
    let mut sum = 0;
    for &value in arr.as_slice() {
        sum += value;
    }
    assert_eq!(sum, 15);
    
    // Test collect
    let collected: Vec<i32> = arr.as_slice().iter().cloned().collect();
    assert_eq!(collected, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_zero_cost_array_cloning() {
    let data = [1, 2, 3, 4, 5];
    let arr = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    
    // Test cloning
    let cloned = arr.clone();
    assert_eq!(cloned.len(), 5);
    assert_eq!(cloned.get(0), Some(&1));
    
    // Test that both arrays work
    assert_eq!(arr.len(), cloned.len());
    assert_eq!(arr.get(0), cloned.get(0));
}

#[test]
fn test_zero_cost_array_performance() {
    // Test performance with many array operations
    let data: Vec<i32> = (0..1000).collect();
    let arr = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    
    // Test iteration performance
    let start = std::time::Instant::now();
    let mut sum = 0;
    for &value in arr.as_slice() {
        sum += value;
    }
    let duration = start.elapsed();
    
    assert_eq!(sum, 499500); // Sum of 0..1000
    assert!(duration.as_millis() < 100); // Should be very fast
}

#[test]
fn test_zero_cost_array_memory_management() {
    // Test memory management
    let data = [1, 2, 3, 4, 5];
    let arr = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    
    // Test that we can create and clone arrays
    let cloned = arr.clone();
    assert_eq!(arr.len(), cloned.len());
    assert_eq!(arr.get(0), cloned.get(0));
    
    // Test that we can create multiple arrays with different data
    let data1 = [10, 20, 30];
    let data2 = [40, 50, 60];
    let arr1 = unsafe { ZeroCostArray::new(data1.as_ptr(), data1.len()) };
    let arr2 = unsafe { ZeroCostArray::new(data2.as_ptr(), data2.len()) };
    
    assert_eq!(arr1.len(), 3);
    assert_eq!(arr2.len(), 3);
    assert_eq!(arr1.get(0), Some(&10));
    assert_eq!(arr2.get(0), Some(&40));
}

#[test]
fn test_zero_cost_array_thread_safety() {
    let data = [1, 2, 3, 4, 5];
    let arr = std::sync::Arc::new(unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) });
    
    // Test thread safety
    let arr1 = arr.clone();
    let arr2 = arr.clone();
    
    let handle1 = std::thread::spawn(move || {
        let mut sum = 0;
        for &value in arr1.as_slice() {
            sum += value;
        }
        sum
    });
    
    let handle2 = std::thread::spawn(move || {
        let mut sum = 0;
        for &value in arr2.as_slice() {
            sum += value;
        }
        sum
    });
    
    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();
    
    assert_eq!(sum1, 15);
    assert_eq!(sum2, 15);
}

#[test]
fn test_zero_cost_array_basic_functionality() {
    // Test basic functionality
    let data = [42];
    let arr = unsafe { ZeroCostArray::new(data.as_ptr(), data.len()) };
    
    // Test that basic operations work
    assert_eq!(arr.len(), 1);
    assert!(!arr.is_empty());
    assert_eq!(arr.get(0), Some(&42));
}

#[test]
fn test_zero_cost_array_edge_cases() {
    // Test edge cases
    let empty_data: [i32; 0] = [];
    let empty_arr = unsafe { ZeroCostArray::new(empty_data.as_ptr(), empty_data.len()) };
    
    assert_eq!(empty_arr.len(), 0);
    assert!(empty_arr.is_empty());
    assert_eq!(empty_arr.get(0), None);
    
    // Test single element
    let single_data = [99];
    let single_arr = unsafe { ZeroCostArray::new(single_data.as_ptr(), single_data.len()) };
    
    assert_eq!(single_arr.len(), 1);
    assert!(!single_arr.is_empty());
    assert_eq!(single_arr.get(0), Some(&99));
    assert_eq!(single_arr.get(1), None);
}