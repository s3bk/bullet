extern crate math;
extern crate hiirc;
extern crate regex;

use hiirc::*;
use math::lang;
use math::eval::{EvalContext, EvalError};
use math::expr::ExprError;
use math::diff::diff;

use std::time::Duration;
use std::sync::Arc;
use regex::Regex;
use std::fmt::{self, Display};

struct Calc {
    re_diff: Regex,
    re_define: Regex,
    re_eval: Regex,
    context: EvalContext
}
impl Calc {
    fn new() -> Calc {
        Calc {
            re_diff: Regex::new(r"^d/d([[:alpha:]]+)\s+(\S.*)").unwrap(),
            re_define: Regex::new(r"^([[:alpha:]]+)\s*:=\s*(\S+)").unwrap(),
            re_eval: Regex::new(r"^eval\s+(\S.*)").unwrap(),
            context: EvalContext::new()
        }
    }
}

static NICKNAME: &'static str = "calc";
static USERNAME: &'static str = "calc";
static REALNAME: &'static str = "A Rust calculator";

enum Error {
    ParseError,
    NotAFloat,
    NotImplemented,
    Eval(EvalError),
    Expr(ExprError),
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Error::ParseError => write!(f, "could not parse the expression"),
           Error::NotAFloat => write!(f, "this is not a number"),
           Error::NotImplemented => write!(f, "too soon …"),
           Error::Eval(ref e) => e.fmt(f),
           Error::Expr(ref e) => e.fmt(f)
       }
    }
}

type RunRes = Option<Result<Option<String>, Error>>;
impl Calc {
    fn run_diff(&self, msg: &str) -> RunRes {
        self.re_diff.captures(msg).map(|cap| {
            let var = cap.get(1).unwrap().as_str();
            let expr_s = cap.get(2).unwrap().as_str();
            
            let expr = lang::parse_Expr(expr_s).map_err(|e| Error::ParseError)?;
            let node = expr.to_node().map_err(|e| Error::Expr(e))?;
            let res = diff(&node, var).simplify();
            Ok(Some(node.simplify().to_string()))
        })
    }

    fn run_define(&mut self, msg: &str) -> RunRes {
        self.re_define.captures(msg).map(|cap| {
            let var = cap.get(1).unwrap().as_str();
            let value = cap.get(2).unwrap().as_str();
                
            let val: f64 = value.parse().map_err(|e| Error::NotAFloat)?;
            self.context.set(var, val);
            Ok(None)
        })
    }

    fn run_eval(&self, msg: &str) -> RunRes {
        self.re_eval.captures(msg).map(|cap| {
            let expr_s = cap.get(1).unwrap().as_str();
            
            let expr = lang::parse_Expr(expr_s).map_err(|e| Error::ParseError)?;
            let node = expr.to_node().map_err(|e| Error::Expr(e))?;
            let r = self.context.eval(&node).map_err(|e| Error::Eval(e))?;
            Ok(Some(r.to_string()))
        })
    }
    
}
impl Listener for Calc {
    fn channel_msg(&mut self, irc: Arc<Irc>, channel: Arc<Channel>, user: Arc<ChannelUser>, msg: &str) {
        if let Some(r) = self.run_diff(msg)
            .or_else(|| self.run_define(msg))
            .or_else(|| self.run_eval(msg))
        {
            match r {
                Ok(Some(s)) => irc.privmsg(channel.name(), &*s).unwrap(),
                Ok(None) => (),
                Err(e) => irc.privmsg(channel.name(), &e.to_string()).unwrap()
            }
        }
    }
    
    fn welcome(&mut self, irc: Arc<Irc>) {
        irc.join("#rust-offtopic", None);
        irc.join("#rust-sci", None);
    }
    
    fn ping(&mut self, irc: Arc<Irc>, server: &str) {
        irc.pong(server);
    }
}

fn main() {
    Settings::new("irc.mozilla.org:6667", NICKNAME)
        .username(USERNAME)
        .realname(REALNAME)
        .reconnection(ReconnectionSettings::Reconnect {
            max_attempts: 0,
            delay_between_attempts: Duration::from_secs(5),
            delay_after_disconnect: Duration::from_secs(15),
        })
        .auto_ping(false)
        .dispatch(Calc::new()).unwrap();
}
