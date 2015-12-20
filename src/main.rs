extern crate telegram_bot;

use telegram_bot::*;

fn main() {
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    let res = listener.listen(|u| {
        if let Some(m) = u.message {
            match m.msg {
                MessageType::Text(t) => {
                    let mut values = t.splitn(2, " ");
                    if let Some(c) = values.next() {
                        match c {
                            "/huy" => { try!(api.send_message(m.chat.id(), format!("{} думает, {} хуй", m.from.first_name, values.next().unwrap_or("Костя")), None, None, None)); },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        Ok(ListeningAction::Continue)
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
