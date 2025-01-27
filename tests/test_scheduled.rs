use pyo3::{
    types::{PyDate, PyList},
    Python,
};
use rstest::rstest;

mod common;
use common::{xirr_expected_result, xnpv_expected_result, PaymentsLoader};

#[rstest]
#[case::unordered("tests/samples/unordered.csv")]
#[case::random_100("tests/samples/random_100.csv")]
fn test_xnpv_samples(#[case] input: &str) {
    let rate = 0.1;
    let result: f64 = Python::with_gil(|py| {
        let payments = PaymentsLoader::from_csv(py, input).to_records();
        pyxirr_call!(py, "xnpv", (rate, payments))
    });
    assert_almost_eq!(result, xnpv_expected_result(rate, input));
}

#[rstest]
#[case::unordered("tests/samples/unordered.csv")]
#[case::single_redemption("tests/samples/single_redemption.csv")]
#[case::random("tests/samples/random.csv")]
#[case::random_100("tests/samples/random_100.csv")]
#[case::random_1000("tests/samples/random_1000.csv")]
#[case::case_30_0("tests/samples/30-0.csv")]
#[case::case_30_1("tests/samples/30-1.csv")]
#[case::case_30_2("tests/samples/30-2.csv")]
#[case::case_30_3("tests/samples/30-3.csv")]
#[case::case_30_4("tests/samples/30-4.csv")]
#[case::case_30_5("tests/samples/30-5.csv")]
#[case::case_30_6("tests/samples/30-6.csv")]
#[case::case_30_7("tests/samples/30-7.csv")]
#[case::case_30_8("tests/samples/30-8.csv")]
#[case::case_30_9("tests/samples/30-9.csv")]
#[case::case_30_10("tests/samples/30-10.csv")]
#[case::case_30_11("tests/samples/30-11.csv")]
#[case::case_30_12("tests/samples/30-12.csv")]
#[case::case_30_13("tests/samples/30-13.csv")]
#[case::case_30_14("tests/samples/30-14.csv")]
#[case::case_30_15("tests/samples/30-15.csv")]
#[case::case_30_16("tests/samples/30-16.csv")]
#[case::case_30_17("tests/samples/30-17.csv")]
#[case::case_30_18("tests/samples/30-18.csv")]
#[case::case_30_19("tests/samples/30-19.csv")]
#[case::case_30_20("tests/samples/30-20.csv")]
#[case::case_30_21("tests/samples/30-21.csv")]
#[case::case_30_22("tests/samples/30-22.csv")]
#[case::case_30_23("tests/samples/30-23.csv")]
#[case::case_30_24("tests/samples/30-24.csv")]
#[case::case_30_25("tests/samples/30-25.csv")]
#[case::case_30_26("tests/samples/30-26.csv")]
#[case::case_30_27("tests/samples/30-27.csv")]
#[case::case_30_28("tests/samples/30-28.csv")]
#[case::case_30_29("tests/samples/30-29.csv")]
#[case::case_30_30("tests/samples/30-30.csv")]
#[case::case_30_31("tests/samples/30-31.csv")]
#[case::case_30_32("tests/samples/30-32.csv")]
#[case::case_30_33("tests/samples/30-33.csv")]
#[case::case_30_34("tests/samples/30-34.csv")]
#[case::case_30_35("tests/samples/30-35.csv")]
#[case::case_30_36("tests/samples/30-36.csv")]
#[case::case_30_37("tests/samples/30-37.csv")]
#[case::case_30_38("tests/samples/30-38.csv")]
#[case::case_30_39("tests/samples/30-39.csv")]
#[case::case_30_40("tests/samples/30-40.csv")]
#[case::case_30_41("tests/samples/30-41.csv")]
#[case::case_30_42("tests/samples/30-42.csv")]
#[case::case_30_43("tests/samples/30-43.csv")]
#[case::case_30_44("tests/samples/30-44.csv")]
#[case::case_30_45("tests/samples/30-45.csv")]
#[case::case_30_46("tests/samples/30-46.csv")]
#[case::case_30_47("tests/samples/30-47.csv")]
#[case::case_30_48("tests/samples/30-48.csv")]
#[case::close_to_minus_0_99("tests/samples/minus_0_99.csv")]
#[case::close_to_minus_0_99999("tests/samples/minus_0_99999.csv")]
fn test_xirr_samples(#[case] input: &str) {
    let result = Python::with_gil(|py| {
        let payments = PaymentsLoader::from_csv(py, input).to_records();
        let rate: Option<f64> = pyxirr_call!(py, "xirr", (payments,));

        if let Some(rate) = rate {
            let xnpv: f64 = pyxirr_call!(py, "xnpv", (rate, payments));
            assert_almost_eq!(xnpv, 0.0, 1e-3);
        }

        rate.unwrap_or(f64::NAN)
    });

    let expected = xirr_expected_result(input);

    if result.is_nan() {
        assert!(expected.is_nan());
    } else {
        assert_almost_eq!(result, expected);
    }
}

#[rstest]
fn test_xirr_silent() {
    Python::with_gil(|py| {
        let args = (PyList::empty(py), PyList::empty(py));
        let err = pyxirr_call_impl!(py, "xirr", args).unwrap_err();
        assert!(err.is_instance(py, py.get_type::<pyxirr::InvalidPaymentsError>()));

        let result: Option<f64> = pyxirr_call!(py, "xirr", args, py_dict!(py, "silent" => true));
        assert!(result.is_none());
    })
}

#[rstest]
fn test_xfv() {
    // http://westclintech.com/SQL-Server-Financial-Functions/SQL-Server-XFV-function
    Python::with_gil(|py| {
        let args = (
            PyDate::new(py, 2011, 2, 1).unwrap(),
            PyDate::new(py, 2011, 3, 1).unwrap(),
            PyDate::new(py, 2012, 2, 1).unwrap(),
            0.00142,
            0.00246,
            100000.,
        );
        let result: f64 = pyxirr_call!(py, "xfv", args);
        assert_almost_eq!(result, 100235.088391894);
    });
}

#[rstest]
fn test_xnfv() {
    Python::with_gil(|py| {
        let payments = PaymentsLoader::from_csv(py, "tests/samples/xnfv.csv").to_records();
        let result: f64 = pyxirr_call!(py, "xnfv", (0.0250, payments));
        assert_almost_eq!(result, 57238.1249299303);
    });
}

#[rstest]
fn test_sum_xfv_eq_xnfv() {
    Python::with_gil(|py| {
        let rate = 0.0250;
        let (dates, amounts) = PaymentsLoader::from_csv(py, "tests/samples/xnfv.csv").to_columns();

        let xnfv_result: f64 = pyxirr_call!(py, "xnfv", (rate, dates, amounts));

        let builtins = py.import("builtins").unwrap();
        let locals = py_dict!(py, "dates" => dates);
        let min_date = py.eval("min(dates)", Some(locals), Some(builtins.dict())).unwrap();
        let max_date = py.eval("max(dates)", Some(locals), Some(builtins.dict())).unwrap();

        let sum_xfv_result: f64 = dates
            .iter()
            .unwrap()
            .map(Result::unwrap)
            .zip(amounts.iter().unwrap().map(Result::unwrap))
            .map(|(date, amount)| -> f64 {
                pyxirr_call!(py, "xfv", (min_date, date, max_date, rate, rate, amount))
            })
            .sum();

        assert_almost_eq!(xnfv_result, sum_xfv_result);
    });
}

// https://www.mathworks.com/help/finance/xirr.html
#[rstest]
#[case("30/360 SIA", 0.100675477282743)] // 1
#[case("act/360", 0.0991988898057063)] // 2
#[case("act/365F", 0.10064378342638)] // 3
#[case("30/360 ISDA", 0.100675477282743)] // 5
#[case("30E/360", 0.100675477282743)] // 6
#[case("act/act ISDA", 0.100739648987346)] // 12
fn test_xirr_day_count(#[case] day_count: &str, #[case] expected: f64) {
    Python::with_gil(|py| {
        let dates = ["01/12/2007", "02/14/2008", "03/03/2008", "06/14/2008", "12/01/2008"];
        let amounts = [-10000, 2500, 2000, 3000, 4000];

        let kwargs = py_dict!(py, "day_count" => day_count);
        let value: f64 = pyxirr_call!(py, "xirr", (dates, amounts), kwargs);

        assert_almost_eq!(value, expected);
    })
}
