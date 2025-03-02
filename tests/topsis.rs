use anyhow::Error;

use proxmox_resource_scheduling::topsis::{rank_alternatives, Criteria, Criterion, Matrix};

#[test]
fn test_topsis_single_criterion() -> Result<(), Error> {
    let criteria_pos = Criteria::new([Criterion::new("the one and only".to_string(), 1.0)])?;

    let criteria_neg = Criteria::new([Criterion::new("the one and only".to_string(), -1.0)])?;

    let matrix = vec![[0.0]];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_pos)?,
        vec![0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_neg)?,
        vec![0],
    );

    let matrix = vec![[0.0], [2.0]];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_pos)?,
        vec![1, 0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_neg)?,
        vec![0, 1],
    );

    let matrix = vec![[1.0], [2.0]];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_pos)?,
        vec![1, 0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_neg)?,
        vec![0, 1],
    );

    let matrix = vec![[1.0], [2.0], [5.0], [3.0], [4.0]];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_pos)?,
        vec![2, 4, 3, 1, 0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_neg)?,
        vec![0, 1, 3, 4, 2],
    );

    let matrix = vec![[-2.0], [-5.0], [-4.0], [1.0], [-3.0]];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_pos)?,
        vec![3, 0, 4, 2, 1],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_neg)?,
        vec![1, 2, 4, 0, 3],
    );

    Ok(())
}

#[test]
fn test_topsis_two_criteria() -> Result<(), Error> {
    let criteria_max_min = Criteria::new([
        Criterion::new("max".to_string(), 1.0),
        Criterion::new("min".to_string(), -1.0),
    ])?;
    let criteria_min_max = Criteria::new([
        Criterion::new("min".to_string(), -1.0),
        Criterion::new("max".to_string(), 1.0),
    ])?;
    let criteria_min_min = Criteria::new([
        Criterion::new("min1".to_string(), -1.0),
        Criterion::new("min2".to_string(), -1.0),
    ])?;
    let criteria_max_max = Criteria::new([
        Criterion::new("max1".to_string(), 1.0),
        Criterion::new("max2".to_string(), 1.0),
    ])?;

    let matrix = vec![[0.0, 0.0]];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_max_min)?,
        vec![0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_min_max)?,
        vec![0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_min_min)?,
        vec![0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_max_max)?,
        vec![0],
    );

    #[rustfmt::skip]
    let matrix = vec![
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, -1.0],
        [1.0, 1.0],
        [0.0, 0.0],
    ];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_max_min)?,
        vec![2, 1, 4, 3, 0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_min_max)?,
        vec![0, 3, 4, 1, 2],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_min_min)?,
        vec![2, 4, 1, 0, 3],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_max_max)?,
        vec![3, 0, 1, 4, 2],
    );

    // This one was cross-checked with https://decision-radar.com/topsis rather than manually.
    #[rustfmt::skip]
    let matrix = vec![
        [7.0, 4.0],
        [1.0, 0.5],
        [-1.0, -1.0],
        [-8.5, 11.5],
        [4.0, 7.0],
    ];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_max_min)?,
        vec![0, 1, 4, 2, 3],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_min_max)?,
        vec![3, 2, 4, 1, 0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_min_min)?,
        vec![2, 3, 1, 0, 4],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_max_max)?,
        vec![4, 0, 1, 3, 2],
    );

    Ok(())
}

#[test]
fn test_topsis_three_criteria() -> Result<(), Error> {
    let criteria_first = Criteria::new([
        Criterion::new("more".to_string(), 10.0),
        Criterion::new("less".to_string(), 0.2),
        Criterion::new("least".to_string(), 0.1),
    ])?;
    let criteria_second = Criteria::new([
        Criterion::new("less".to_string(), 0.2),
        Criterion::new("more".to_string(), 10.0),
        Criterion::new("least".to_string(), 0.1),
    ])?;
    let criteria_third = Criteria::new([
        Criterion::new("less".to_string(), 0.2),
        Criterion::new("least".to_string(), 0.1),
        Criterion::new("more".to_string(), 10.0),
    ])?;
    let criteria_min = Criteria::new([
        Criterion::new("most".to_string(), -10.0),
        Criterion::new("more".to_string(), -5.0),
        Criterion::new("less".to_string(), 0.1),
    ])?;

    #[rustfmt::skip]
    let matrix = vec![
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_first)?,
        vec![0, 1, 2],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_second)?,
        vec![1, 0, 2],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_third)?,
        vec![2, 0, 1],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_min)?,
        vec![2, 1, 0],
    );

    #[rustfmt::skip]
    let matrix = vec![
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1000.0],
    ];
    // normalization ensures that it's still the same as above
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_first)?,
        vec![0, 1, 2],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_second)?,
        vec![1, 0, 2],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_third)?,
        vec![2, 0, 1],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_min)?,
        vec![2, 1, 0],
    );

    #[rustfmt::skip]
    let matrix = vec![
        [-1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_first)?,
        vec![2, 1, 0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_second)?,
        vec![2, 0, 1],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix.clone())?, &criteria_third)?,
        vec![2, 1, 0],
    );
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_min)?,
        vec![0, 1, 2],
    );

    Ok(())
}

#[test]
fn test_nan() {
    #[rustfmt::skip]
    let matrix = vec![
        [-1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, f64::NAN, 1.0],
    ];
    assert!(Matrix::new(matrix).is_err());
}

#[test]
fn test_zero() -> Result<(), Error> {
    let criteria_zero = Criteria::new([
        Criterion::new("z".to_string(), 0.0),
        Criterion::new("e".to_string(), 0.0),
        Criterion::new("ro".to_string(), 0.0),
    ]);
    assert!(criteria_zero.is_err());

    let criteria_first = Criteria::new([
        Criterion::new("more".to_string(), 10.0),
        Criterion::new("less".to_string(), 0.2),
        Criterion::new("least".to_string(), 0.1),
    ])?;

    #[rustfmt::skip]
    let matrix = vec![
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    assert_eq!(
        rank_alternatives(&Matrix::new(matrix)?, &criteria_first)?,
        vec![1, 2, 0],
    );

    Ok(())
}
