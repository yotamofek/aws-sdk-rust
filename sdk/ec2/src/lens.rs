pub(crate) fn reflens_describe_instances_output_output_next_token(
    input: &crate::operation::describe_instances::DescribeInstancesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
pub(crate) fn lens_describe_instances_output_output_reservations(
    input: crate::operation::describe_instances::DescribeInstancesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Reservation>> {
    let input = match input.reservations {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
