use headless_chrome::Browser;
use headless_chrome::LaunchOptions;
use std::thread;

fn build_launch_opts() -> headless_chrome::LaunchOptions<'static> {
    LaunchOptions::default_builder()
        .headless(false)
        .window_size(Some((1920, 1080)))
        .build()
        .expect("failed build")
}

fn main() -> anyhow::Result<()> {
    let launch_opts = build_launch_opts();
    let browser = Browser::new(launch_opts)?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://connpass.com/")?
        .wait_for_element("#id_q")?
        .click()?;

    tab.type_str("宇都宮")?;

    tab.wait_for_element("#header_event_search_form > div > button")?
        .click()?;

    wait();

    let elements = tab.wait_for_elements("#main > div.event_area > div")?;

    for element in elements {
        let inner_text = element
            .wait_for_element("span.series_title")?
            .get_inner_text()?;

        if inner_text == "宮もく".to_owned() {
            let year = element
                .wait_for_element("div.event_schedule_area > p.year")?
                .get_inner_text()?;

            let date = element
                .wait_for_element("div.event_schedule_area > p.date")?
                .get_inner_text()?;

            let time = element
                .wait_for_element("div.event_label_area > p.time")?
                .get_inner_text()?;

            let title = element
                .wait_for_element("div.event_inner > p.event_title > a")?
                .get_inner_text()?;

            println!("{:?} {:?} {:?} {:?}", year, date, time, title);
        }
    }

    Ok(())
}

fn wait() {
    thread::sleep(std::time::Duration::from_millis(2000));
}
