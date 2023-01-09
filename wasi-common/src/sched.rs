use crate::clocks::WasiMonotonicClock;
use crate::stream::WasiStream;
use crate::Error;
pub mod subscription;
pub use cap_std::time::Duration;

pub use subscription::{
    MonotonicClockSubscription, RwEventFlags, RwSubscription, RwSubscriptionKind, Subscription,
    SubscriptionResult,
};

#[async_trait::async_trait]
pub trait WasiSched: Send + Sync {
    async fn poll_oneoff<'a>(&self, poll: &mut Poll<'a>) -> Result<(), Error>;
    async fn sched_yield(&self) -> Result<(), Error>;
    async fn sleep(&self, duration: Duration) -> Result<(), Error>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Userdata(u64);
impl From<u64> for Userdata {
    fn from(u: u64) -> Userdata {
        Userdata(u)
    }
}

impl From<Userdata> for u64 {
    fn from(u: Userdata) -> u64 {
        u.0
    }
}

pub type PollResults = Vec<(SubscriptionResult, Userdata)>;

pub struct Poll<'a> {
    subs: Vec<(Subscription<'a>, Userdata)>,
}

impl<'a> Poll<'a> {
    pub fn new() -> Self {
        Self { subs: Vec::new() }
    }
    pub fn subscribe_monotonic_clock(
        &mut self,
        clock: &'a dyn WasiMonotonicClock,
        deadline: u64,
        absolute: bool,
        ud: Userdata,
    ) -> Result<(), Error> {
        let deadline = if absolute {
            // Convert an absolute deadline to a relative one.
            deadline.saturating_sub(clock.now())
        } else {
            deadline
        };
        self.subs.push((
            Subscription::MonotonicClock(MonotonicClockSubscription { clock, deadline }),
            ud,
        ));
        Ok(())
    }
    pub fn subscribe_read(&mut self, stream: &'a dyn WasiStream, ud: Userdata) {
        self.subs.push((
            Subscription::ReadWrite(RwSubscription::new(stream), RwSubscriptionKind::Read),
            ud,
        ));
    }
    pub fn subscribe_write(&mut self, stream: &'a dyn WasiStream, ud: Userdata) {
        self.subs.push((
            Subscription::ReadWrite(RwSubscription::new(stream), RwSubscriptionKind::Write),
            ud,
        ));
    }
    pub fn results(self) -> impl Iterator<Item = (SubscriptionResult, Userdata)> + 'a {
        self.subs
            .into_iter()
            .filter_map(|(s, ud)| SubscriptionResult::from_subscription(s).map(|r| (r, ud)))
    }
    pub fn is_empty(&self) -> bool {
        self.subs.is_empty()
    }
    pub fn earliest_clock_deadline(&self) -> Option<&MonotonicClockSubscription<'a>> {
        self.subs
            .iter()
            .filter_map(|(s, _ud)| match s {
                Subscription::MonotonicClock(t) => Some(t),
                _ => None,
            })
            .min_by(|a, b| a.deadline.cmp(&b.deadline))
    }
    pub fn rw_subscriptions<'b>(
        &'b mut self,
    ) -> impl Iterator<Item = (&'b mut RwSubscription<'a>, RwSubscriptionKind)> {
        self.subs.iter_mut().filter_map(|(s, _ud)| match s {
            Subscription::ReadWrite(sub, kind) => Some((sub, *kind)),
            _ => None,
        })
    }
}
