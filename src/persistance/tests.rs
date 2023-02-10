mod answers_tests {
    use sqlx::PgPool;

    use crate::{
        models::{Answer, DBError, Question},
        persistance::{
            answers_dao::{AnswersDao, AnswersDaoImpl},
            questions_dao::{QuestionsDao, QuestionsDaoImpl},
        },
    };

#[sqlx::test]
async fn create_answer_should_fail_with_malformed_uuid(pool: PgPool) -> Result<(), String> {
    let answers_dao = AnswersDaoImpl::new(pool);

    let result = answers_dao.create_answer(Answer {question_uuid: "malformed".to_owned(), content: "test cotent".to_owned()}).await;

    if result.is_ok() {
        return Err(format!("Expected an error but got the following result: {:?}",
        result.unwrap()
    ));
    }

    if let Err(DBError::InvalidUUID(_)) = result {
        Ok(())
    } else {
        Err(format!(
            "Expected an invalid UUID error but got the following error: {:?}",
            result.err()
        ))
    }

    }

}
