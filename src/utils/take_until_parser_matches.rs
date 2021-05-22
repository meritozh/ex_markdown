/// port from: https://github.com/Geal/nom/pull/469
use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult, InputIter, InputLength, InputTake, Parser,
};

pub(crate) fn take_until_parser_matches<F, Input, O, Error>(
    mut f: F,
) -> impl FnMut(Input) -> IResult<Input, Input, Error>
where
    Input: InputTake + InputIter + InputLength + Clone,
    F: Parser<Input, O, Error>,
    Error: ParseError<Input>,
{
    move |input: Input| {
        let i = input.clone();
        for (ind, _) in i.iter_indices() {
            let (remaining, _taken) = i.take_split(ind);
            match f.parse(remaining) {
                Err(_) => (),
                Ok(_) => {
                    let res: IResult<Input, Input, Error> = Ok(i.take_split(ind));
                    return res;
                }
            }
        }
        // Attempt to match one last time past the end of the input. This
        // allows for 0-length combinators to be used (for example, an eof
        // combinator).
        let (remaining, _taken) = i.take_split(i.input_len());
        match f.parse(remaining) {
            Err(_) => (),
            Ok(_) => {
                let res: IResult<Input, Input, Error> = Ok(i.take_split(i.input_len()));
                return res;
            }
        }
        Err(Err::Error(Error::from_error_kind(i, ErrorKind::Eof)))
    }
}

pub(crate) fn take_until_parser_matches_and_consume<F, Input, O, Error>(
    mut f: F,
) -> impl FnMut(Input) -> IResult<Input, Input, Error>
where
    Input: InputTake + InputIter + InputLength + Clone,
    F: Parser<Input, O, Error>,
    Error: ParseError<Input>,
{
    move |input: Input| {
        let i = input.clone();
        for (ind, _) in i.iter_indices() {
            let (remaining, taken) = i.take_split(ind);
            match f.parse(remaining) {
                Err(_) => (),
                Ok((inner_remaining, _parser_result)) => {
                    let res: IResult<Input, Input, Error> = Ok((inner_remaining, taken));
                    return res;
                }
            }
        }
        // Attempt to match one last time past the end of the input. This
        // allows for 0-length combinators to be used (for example, an eof
        // combinator).
        let (remaining, taken) = i.take_split(i.input_len());
        match f.parse(remaining) {
            Err(_) => (),
            Ok((inner_remaining, _parser_result)) => {
                let res: IResult<Input, Input, Error> = Ok((inner_remaining, taken));
                return res;
            }
        }
        Err(Err::Error(Error::from_error_kind(i, ErrorKind::Eof)))
    }
}
