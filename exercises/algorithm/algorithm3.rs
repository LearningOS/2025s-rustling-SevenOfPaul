/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn mergeSort<T>(array: Vec<T>) -> Vec<T> where T: Clone+PartialOrd+Copy {
    if array.len() < 2 {
        return array
    }
    let mid= array.len()>>1;
    return merge(mergeSort(array[0..mid].to_vec()),mergeSort(array[mid.. array.len()].to_vec()));
}
fn merge<T>(leftV:Vec<T>,rightV:Vec<T>)->Vec<T> where T: Clone+PartialOrd+Copy{
    let mut arr=vec![];
    let mut left=0;
    let mut right=0;
    while left<leftV.len() && right< rightV.len(){
        if leftV[left]<rightV[right]{
            arr.push(leftV[left]);
            left+=1;
        }else{
            arr.push(rightV[right]);
            right+=1;
        }
    }
    arr.extend_from_slice(&leftV[left..leftV.len()]);
    arr.extend_from_slice(&rightV[right..rightV.len()]);
 return arr
}
fn sort<T:Clone+PartialOrd+Copy>(array: &mut [T]){
    array.copy_from_slice(&mergeSort(array.to_vec()));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}