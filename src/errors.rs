error_chain!{
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        TimeOut(::r2d2::GetTimeout);
        Diesel(::diesel::result::Error);
    }
}