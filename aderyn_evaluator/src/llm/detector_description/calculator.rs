use std::{collections::HashMap, error::Error};

use aderyn_driver::detector::Detector;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

use crate::core::{elo, Battle};

pub async fn run(
    contestants: HashMap<String, Box<dyn Detector>>,
    satisfiers: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let client = make_client();
    let mut battles = vec![];

    for satisfier in satisfiers {
        for contestant_a in contestants.keys() {
            for contestant_b in contestants.keys() {
                if contestant_a == contestant_b {
                    continue;
                }
                let contestant_a_desc = contestants[contestant_a].description();
                let contestant_b_desc = contestants[contestant_b].description();

                let request = make_request(&contestant_a_desc, &contestant_b_desc, &satisfier)?;

                let plausible_winner = fire_request(&client, request).await;

                match plausible_winner {
                    Ok(winner) => {
                        let battle = match winner {
                            WinnerByLanguageModelDecision::ContestantA => Battle {
                                first_contestant_id: contestant_a.clone(),
                                second_contestant_id: contestant_b.clone(),
                                first_contestant_score: 1.0,
                                second_contestant_score: 0.0,
                            },
                            WinnerByLanguageModelDecision::ContestantB => Battle {
                                first_contestant_id: contestant_a.clone(),
                                second_contestant_id: contestant_b.clone(),
                                first_contestant_score: 0.0,
                                second_contestant_score: 1.0,
                            },
                            WinnerByLanguageModelDecision::Tie => Battle {
                                // (both good or both bad)
                                first_contestant_id: contestant_a.clone(),
                                second_contestant_id: contestant_b.clone(),
                                first_contestant_score: 0.5,
                                second_contestant_score: 0.5,
                            },
                        };
                        battles.push(battle);
                    }
                    Err(_) => {
                        println!("Skipping !")
                    }
                };
            }
        }
    }

    let computed_scores = elo::score(&battles);
    println!("Scores: \n{:?}", computed_scores);

    Ok(())
}

enum WinnerByLanguageModelDecision {
    ContestantA,
    ContestantB,
    Tie,
}

async fn fire_request(
    client: &Client<OpenAIConfig>,
    request: CreateChatCompletionRequest,
) -> Result<WinnerByLanguageModelDecision, Box<dyn Error>> {
    // println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    let content = response.choices[0]
        .message
        .content
        .clone()
        .ok_or("The response message was corrupted")?;
    println!("{}", content);

    let last_line = content
        .lines()
        .last()
        .ok_or("The response message lacks content")?;

    let last_line_digit: usize = last_line.parse()?;

    let winner = match last_line_digit {
        0 => WinnerByLanguageModelDecision::ContestantA,
        1 => WinnerByLanguageModelDecision::ContestantB,
        2 => WinnerByLanguageModelDecision::Tie,
        _ => {
            return Err("Couldn't decide winner by the output".into());
        }
    };

    Ok(winner)
}

fn make_client() -> Client<OpenAIConfig> {
    let backoff = backoff::ExponentialBackoffBuilder::new()
        .with_max_elapsed_time(Some(std::time::Duration::from_secs(60)))
        .build();

    let client = Client::new().with_backoff(backoff);

    client
}

fn make_request(
    contestant_a_desc: &str,
    contestant_b_desc: &str,
    satisfier: &str,
) -> Result<CreateChatCompletionRequest, Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(include_str!("system.prompt"))
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(include_str!("sample.user.prompt"))
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(include_str!("sample.assistant.prompt"))
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(format!(
                    include_str!("template.user.prompt"),
                    contestant_a_submission = contestant_a_desc,
                    contestant_b_submission = contestant_b_desc,
                    satisfactory_condition = satisfier,
                ))
                .build()?
                .into(),
        ])
        .build()?;

    Ok(request)
}
