#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use screenpipe_db::{
        AudioDevice, ContentType, DatabaseManager, DeviceType, Frame, OcrEngine, SearchResult,
    };

    async fn setup_test_db() -> DatabaseManager {
        // Initialize tracing for debug output
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();

        let db = DatabaseManager::new("sqlite::memory:").await.unwrap();

        // Run all migrations with better error handling
        match sqlx::migrate!("./src/migrations").run(&db.pool).await {
            Ok(_) => {
                tracing::debug!("Database migrations completed successfully");
            }
            Err(e) => {
                eprintln!("Migration error: {:?}", e);
                panic!("Database migration failed: {}", e);
            }
        }

        db
    }

    #[tokio::test]
    async fn test_insert_and_search_ocr() {
        let db = setup_test_db().await;
        let _ = db
            .insert_video_chunk("test_video.mp4", "test_device")
            .await
            .unwrap();
        let frame_id = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id,
            "Hello, world!",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        let results = db
            .search(
                "Hello",
                ContentType::OCR,
                100,
                0,
                None,
                None,
                Some("test"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        if let SearchResult::OCR(ocr_result) = &results[0] {
            assert_eq!(ocr_result.ocr_text, "Hello, world!");
            assert_eq!(ocr_result.file_path, "test_video.mp4");
        } else {
            panic!("Expected OCR result");
        }
    }

    #[tokio::test]
    async fn test_insert_and_search_audio() {
        let db = setup_test_db().await;
        let audio_chunk_id = db.insert_audio_chunk("test_audio.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "Hello from audio",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let em_results = db
            .search(
                "audio",
                ContentType::Audio,
                100,
                0,
                None,
                None,
                Some("test"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(em_results.len(), 0);

        let results = db
            .search(
                "audio",
                ContentType::Audio,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        if let SearchResult::Audio(audio_result) = &results[0] {
            assert_eq!(audio_result.transcription, "Hello from audio");
            assert_eq!(audio_result.file_path, "test_audio.mp4");
        } else {
            panic!("Expected Audio result");
        }
    }

    #[tokio::test]
    async fn test_update_and_search_audio() {
        let db = setup_test_db().await;
        let audio_chunk_id = db.insert_audio_chunk("test_audio.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "Hello from audio",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let a = db
            .update_audio_transcription(audio_chunk_id, "This is a test.")
            .await
            .unwrap();

        assert_eq!(a, 1);

        let em_results = db
            .search(
                "",
                ContentType::Audio,
                100,
                0,
                None,
                None,
                Some("app"),
                Some("window"),
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(em_results.len(), 0);

        let results = db
            .search(
                "",
                ContentType::Audio,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        if let SearchResult::Audio(audio_result) = &results[0] {
            assert_eq!(audio_result.transcription, "This is a test.");
            assert_eq!(audio_result.file_path, "test_audio.mp4");
        } else {
            panic!("Expected Audio result");
        }
    }

    #[tokio::test]
    async fn test_search_all() {
        let db = setup_test_db().await;

        // Insert OCR data
        let _ = db
            .insert_video_chunk("test_video.mp4", "test_device")
            .await
            .unwrap();
        let frame_id = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();

        // Debug: Check if app_name was inserted correctly
        let frame_data: Frame = sqlx::query_as("SELECT * FROM frames WHERE id = ?")
            .bind(frame_id)
            .fetch_one(&db.pool)
            .await
            .unwrap();
        println!("Inserted frame data: {:?}", frame_data);

        db.insert_ocr_text(
            frame_id,
            "Hello from OCR",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        // Verify that frames_fts was populated
        let fts_data: Option<(i64, String, String, String, bool)> = sqlx::query_as(
            "SELECT rowid, browser_url, app_name, window_name, focused FROM frames_fts WHERE rowid = ?",
        )
        .bind(frame_id)
        .fetch_optional(&db.pool)
        .await
        .unwrap();
        println!("Frames FTS data: {:?}", fts_data);

        // Insert Audio data
        let audio_chunk_id = db.insert_audio_chunk("test_audio.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "Hello from audio",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let one_result = db
            .search(
                "Hello",
                ContentType::All,
                100,
                0,
                None,
                None,
                Some("test"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(one_result.len(), 1);

        let results = db
            .search(
                "Hello",
                ContentType::All,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 2);

        let ocr_count = results
            .iter()
            .filter(|r| matches!(r, SearchResult::OCR(_)))
            .count();
        let audio_count = results
            .iter()
            .filter(|r| matches!(r, SearchResult::Audio(_)))
            .count();

        assert_eq!(ocr_count, 1);
        assert_eq!(audio_count, 1);
    }

    #[tokio::test]
    async fn test_search_with_time_range() {
        let db = setup_test_db().await;

        let start_time = Utc::now();

        // Insert OCR data
        let _ = db
            .insert_video_chunk("test_video.mp4", "test_device")
            .await
            .unwrap();
        let frame_id1 = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id1,
            "Hello from OCR 1",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        // Insert first audio data
        let audio_chunk_id = db.insert_audio_chunk("test_audio.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "Hello from audio 1",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // Wait for a short time to ensure timestamp difference
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let mid_time = Utc::now();

        // Wait for another short time
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Insert remaining data
        let frame_id2 = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id2,
            "Hello from OCR 2",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        let raw_ocr_text: Vec<(String, Option<i64>)> =
            sqlx::query_as("SELECT text, frame_id FROM ocr_text")
                .fetch_all(&db.pool)
                .await
                .unwrap();
        println!("Raw OCR text in DB: {:?}", raw_ocr_text);
        // print raw frames with timestamp
        let raw_frames: Vec<(Option<i64>, Option<String>)> =
            sqlx::query_as("SELECT id, timestamp FROM frames")
                .fetch_all(&db.pool)
                .await
                .unwrap();
        println!("Raw frames in DB: {:?}", raw_frames);
        // Check if OCR text is properly indexed in FTS
        let ocr_fts_data: Vec<(i64, String)> =
            sqlx::query_as("SELECT rowid, text FROM ocr_text_fts")
                .fetch_all(&db.pool)
                .await
                .unwrap();
        println!("OCR FTS data: {:?}", ocr_fts_data);

        // check if frames_fts is properly indexed
        let frame_fts_data: Vec<(i64, String, String, String, bool)> = sqlx::query_as(
            "SELECT id, browser_url, app_name, window_name, focused FROM frames_fts",
        )
        .fetch_all(&db.pool)
        .await
        .unwrap();
        println!("Frames FTS data: {:?}", frame_fts_data);

        let insert_result = db
            .insert_audio_transcription(
                audio_chunk_id,
                "Hello from audio 2",
                1,
                "",
                &AudioDevice {
                    name: "test".to_string(),
                    device_type: DeviceType::Output,
                },
                None,
                None,
                None,
            )
            .await;
        println!("Second audio insert result: {:?}", insert_result);

        let raw_transcriptions: Vec<(String, Option<i64>)> =
            sqlx::query_as("SELECT transcription, speaker_id FROM audio_transcriptions")
                .fetch_all(&db.pool)
                .await
                .unwrap();
        println!("Raw transcriptions in DB: {:?}", raw_transcriptions);

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // After inserting both audio transcriptions, let's check all audio entries
        let all_audio = db
            .search_audio("", 100, 0, None, None, None, None, None)
            .await
            .unwrap();
        println!("All audio entries: {:?}", all_audio);

        // Then try specific search
        let audio_results = db
            .search_audio("2", 100, 0, None, None, None, None, None)
            .await
            .unwrap();
        println!("Audio results for '2': {:?}", audio_results);

        let end_time = Utc::now();

        // Debug OCR search with time range
        let ocr_results = db
            .search(
                "Hello",
                ContentType::OCR,
                100,
                0,
                Some(start_time),
                Some(end_time),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        println!("OCR time range results: {:?}", ocr_results);

        assert_eq!(ocr_results.len(), 2);

        // Test search with full time range
        let results = db
            .search(
                "Hello",
                ContentType::All,
                100,
                0,
                Some(start_time),
                Some(end_time),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        println!("Full time range results: {:?}", results);
        assert_eq!(results.len(), 4, "Expected 4 results for full time range");

        // Test search with limited time range
        let results = db
            .search(
                "Hello",
                ContentType::All,
                100,
                0,
                Some(mid_time),
                Some(end_time),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        println!("Limited time range results: {:?}", results);
        assert_eq!(
            results.len(),
            2,
            "Expected 2 results for limited time range"
        );

        // Test search with OCR content type and time range
        let results = db
            .search(
                "Hello",
                ContentType::OCR,
                100,
                0,
                Some(start_time),
                Some(end_time),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 2);

        // Test search with Audio content type and time range
        let results = db
            .search(
                "Hello",
                ContentType::Audio,
                100,
                0,
                Some(start_time),
                Some(end_time),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_count_search_results_with_time_range() {
        let db = setup_test_db().await;

        let start_time = Utc::now();

        // Insert OCR data
        let _ = db
            .insert_video_chunk("test_video.mp4", "test_device")
            .await
            .unwrap();
        let frame_id1 = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id1,
            "Hello from OCR 1",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        // Insert first audio data
        let audio_chunk_id = db.insert_audio_chunk("test_audio.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "Hello from audio 1",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // Capture mid_time after inserting half of the data
        let mid_time = Utc::now();

        // Wait for a short time to ensure timestamp difference
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Insert remaining data
        let frame_id2 = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id2,
            "Hello from OCR 2",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        let audio_chunk_id2 = db.insert_audio_chunk("test_audio2.mp4").await.unwrap();

        db.insert_audio_transcription(
            audio_chunk_id2,
            "Hello from audio 2",
            1,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let end_time = Utc::now();

        // Test search with limited time range
        let results = db
            .search(
                "Hello",
                ContentType::All,
                100,
                0,
                Some(mid_time),
                Some(end_time),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        println!("Limited time range results: {:?}", results);
        assert_eq!(
            results.len(),
            2,
            "Expected 2 results for limited time range"
        );

        // Test count with Audio content type and time range
        let count = db
            .count_search_results(
                "Hello",
                ContentType::Audio,
                Some(start_time),
                Some(end_time),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_insert_and_search_speaker() {
        let db = setup_test_db().await;

        let mut speaker_ids = Vec::new();
        for i in 0..5 {
            let sample_embedding = vec![0.1 * (i as f32 + 1.0); 512];
            let speaker = db.insert_speaker(&sample_embedding).await.unwrap();
            speaker_ids.push(speaker.id);
        }
        let speaker_id = speaker_ids[0];
        assert_eq!(speaker_id, 1);

        let sample_embedding = vec![0.1; 512];
        let speaker = db
            .get_speaker_from_embedding(&sample_embedding)
            .await
            .unwrap();
        assert_eq!(speaker.unwrap().id, 1);
    }

    #[tokio::test]
    async fn test_update_speaker_metadata() {
        let db = setup_test_db().await;

        let sample_embedding = vec![0.1; 512];
        let speaker = db.insert_speaker(&sample_embedding).await.unwrap();
        assert_eq!(speaker.id, 1);

        db.update_speaker_metadata(speaker.id, "test metadata")
            .await
            .unwrap();

        // Add verification
        let speaker = db.get_speaker_by_id(speaker.id).await.unwrap();
        assert_eq!(speaker.metadata, "test metadata");
    }

    #[tokio::test]
    async fn test_get_speaker_by_id() {
        let db = setup_test_db().await;

        let sample_embedding = vec![0.1; 512];
        let speaker = db.insert_speaker(&sample_embedding).await.unwrap();
        assert_eq!(speaker.id, 1);

        let speaker = db.get_speaker_by_id(speaker.id).await.unwrap();
        assert_eq!(speaker.id, 1);
    }

    #[tokio::test]
    async fn test_update_speaker_name() {
        let db = setup_test_db().await;

        let sample_embedding = vec![0.1; 512];
        let speaker = db.insert_speaker(&sample_embedding).await.unwrap();
        assert_eq!(speaker.id, 1);

        db.update_speaker_name(speaker.id, "test name")
            .await
            .unwrap();

        let speaker = db.get_speaker_by_id(speaker.id).await.unwrap();

        println!("Speaker: {:?}", speaker);
        assert_eq!(speaker.name, "test name");
    }

    #[tokio::test]
    async fn test_get_unnamed_speakers() {
        let db = setup_test_db().await;

        // insert n audio chunks for each speaker
        for n in 0..3 {
            let speaker = db.insert_speaker(&vec![n as f32; 512]).await.unwrap();
            for i in 0..=n {
                let audio_chunk_id = db
                    .insert_audio_chunk(&format!("audio{}{}", n, i))
                    .await
                    .unwrap();
                db.insert_audio_transcription(
                    audio_chunk_id,
                    "test transcription",
                    0,
                    "",
                    &AudioDevice {
                        name: "test".to_string(),
                        device_type: DeviceType::Output,
                    },
                    Some(speaker.id),
                    None,
                    None,
                )
                .await
                .unwrap();
            }
        }

        // insert a speaker with a name
        let speaker = db.insert_speaker(&vec![0.1; 512]).await.unwrap();
        db.update_speaker_name(speaker.id, "test name")
            .await
            .unwrap();

        // Get unnamed speakers
        let unnamed_speakers = db.get_unnamed_speakers(10, 0, None).await.unwrap();

        assert_eq!(unnamed_speakers.len(), 3, "Should find 3 unnamed speakers");

        let speaker_3 = &unnamed_speakers[0];
        assert_eq!(speaker_3.id, 3);
        assert!(speaker_3.name.is_empty());

        // speaker 2 should be next
        let speaker_2 = &unnamed_speakers[1];
        assert_eq!(speaker_2.id, 2);
        assert!(speaker_2.name.is_empty());

        // speaker 1 should be last
        let speaker_1 = &unnamed_speakers[2];
        assert_eq!(speaker_1.id, 1);
        assert!(speaker_1.name.is_empty());

        let metadata: serde_json::Value =
            serde_json::from_str(&speaker_3.metadata).expect("Metadata should be valid JSON");

        let audio_samples = metadata["audio_samples"]
            .as_array()
            .expect("Audio Samples should be an array");

        println!("Audio samples: {:?}", audio_samples);

        assert_eq!(audio_samples.len(), 3);
    }

    #[tokio::test]
    async fn test_get_unnamed_speakers_with_speaker_ids() {
        let db = setup_test_db().await;

        // insert n audio chunks for each speaker
        for n in 0..3 {
            let speaker = db.insert_speaker(&vec![n as f32; 512]).await.unwrap();
            for i in 0..=n {
                let audio_chunk_id = db
                    .insert_audio_chunk(&format!("audio{}{}", n, i))
                    .await
                    .unwrap();
                db.insert_audio_transcription(
                    audio_chunk_id,
                    "test transcription",
                    0,
                    "",
                    &AudioDevice {
                        name: "test".to_string(),
                        device_type: DeviceType::Output,
                    },
                    Some(speaker.id),
                    None,
                    None,
                )
                .await
                .unwrap();
            }
        }

        // insert a speaker with a name
        let speaker = db.insert_speaker(&vec![0.1; 512]).await.unwrap();
        db.update_speaker_name(speaker.id, "test name")
            .await
            .unwrap();

        // Get unnamed speakers
        let unnamed_speakers = db
            .get_unnamed_speakers(10, 0, Some(vec![speaker.id, 1, 2, 3]))
            .await
            .unwrap();

        assert_eq!(unnamed_speakers.len(), 3, "Should find 3 unnamed speakers");
        // ensure the order is correct
        assert_eq!(unnamed_speakers[0].id, 3);
        assert_eq!(unnamed_speakers[1].id, 2);
        assert_eq!(unnamed_speakers[2].id, 1);
    }

    #[tokio::test]
    async fn test_merge_speakers() {
        let db = setup_test_db().await;

        let speaker_1 = db.insert_speaker(&vec![0.1; 512]).await.unwrap();
        db.update_speaker_name(speaker_1.id, "speaker 1")
            .await
            .unwrap();
        let speaker_2 = db.insert_speaker(&vec![0.2; 512]).await.unwrap();
        db.update_speaker_name(speaker_2.id, "speaker 2")
            .await
            .unwrap();

        // for each speaker, insert 2 audio chunks
        for speaker in [speaker_1.clone(), speaker_2.clone()] {
            for i in 0..2 {
                let audio_chunk_id = db
                    .insert_audio_chunk(&format!("audio{}{}", speaker.id, i))
                    .await
                    .unwrap();

                // insert audio transcription
                db.insert_audio_transcription(
                    audio_chunk_id,
                    "test transcription",
                    0,
                    "",
                    &AudioDevice {
                        name: "test".to_string(),
                        device_type: DeviceType::Output,
                    },
                    Some(speaker.id),
                    None,
                    None,
                )
                .await
                .unwrap();
            }
        }

        db.merge_speakers(speaker_1.id, speaker_2.id).await.unwrap();

        let speakers = db.search_speakers("").await.unwrap();
        assert_eq!(speakers.len(), 1);
        assert_eq!(speakers[0].name, "speaker 1");
    }

    #[tokio::test]
    async fn test_search_speakers() {
        let db = setup_test_db().await;

        let speaker = db.insert_speaker(&vec![0.1; 512]).await.unwrap();
        db.update_speaker_name(speaker.id, "test name")
            .await
            .unwrap();

        let speakers = db.search_speakers("test").await.unwrap();
        assert_eq!(speakers.len(), 1);
        assert_eq!(speakers[0].name, "test name");
    }

    #[tokio::test]
    async fn test_delete_speaker() {
        let db = setup_test_db().await;

        let speaker = db.insert_speaker(&vec![0.1; 512]).await.unwrap();

        let audio_chunk_id = db.insert_audio_chunk("test_audio.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "test transcription",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            Some(speaker.id),
            None,
            None,
        )
        .await
        .unwrap();

        db.delete_speaker(speaker.id).await.unwrap();

        let speakers = db.search_speakers("").await.unwrap();
        assert_eq!(speakers.len(), 0);

        // make sure audio_chunks are deleted
        let audio_chunks = db.get_audio_chunks_for_speaker(speaker.id).await.unwrap();
        assert_eq!(audio_chunks.len(), 0);
    }

    #[tokio::test]
    async fn test_mark_speaker_as_hallucination() {
        let db = setup_test_db().await;

        let speaker = db.insert_speaker(&vec![0.1; 512]).await.unwrap();
        db.mark_speaker_as_hallucination(speaker.id).await.unwrap();

        let speakers = db.search_speakers("").await.unwrap();
        assert_eq!(speakers.len(), 0);
    }

    #[tokio::test]
    async fn test_get_similar_speakers() {
        let db = setup_test_db().await;

        // Create first speaker with audio data
        let speaker = db.insert_speaker(&vec![0.1; 512]).await.unwrap();
        db.update_speaker_name(speaker.id, "test name")
            .await
            .unwrap();
        let audio_chunk_id = db.insert_audio_chunk("test_audio1.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "test transcription",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            Some(speaker.id),
            None,
            None,
        )
        .await
        .unwrap();

        // Create second speaker with audio data
        let speaker2 = db.insert_speaker(&vec![0.2; 512]).await.unwrap();
        db.update_speaker_name(speaker2.id, "name").await.unwrap();
        let audio_chunk_id2 = db.insert_audio_chunk("test_audio2.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id2,
            "test transcription",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            Some(speaker2.id),
            None,
            None,
        )
        .await
        .unwrap();

        let similar_speakers = db.get_similar_speakers(speaker.id, 10).await.unwrap();
        assert_eq!(similar_speakers.len(), 1);
        assert_eq!(similar_speakers[0].id, speaker2.id);
    }

    #[tokio::test]
    async fn test_search_with_frame_name() {
        let db = setup_test_db().await;

        // Insert video chunk and frames
        let _ = db
            .insert_video_chunk("test_video.mp4", "test_device")
            .await
            .unwrap();

        // Insert first frame with OCR
        let frame_id1 = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id1,
            "Hello from frame 1",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        // Insert second frame with OCR
        let frame_id2 = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id2,
            "Hello from frame 2",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        // Test searching OCR with frame_name filter
        let results = db
            .search(
                "text:Hello",
                ContentType::OCR,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("test_video"),
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(
            results.len(),
            2,
            "Should find both frames with matching video path"
        );

        // Test searching OCR with non-matching frame_name
        let results = db
            .search(
                "Hello",
                ContentType::OCR,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("non_existent"),
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(
            results.len(),
            0,
            "Should find no frames with non-matching path"
        );

        // Test searching All content with frame_name filter
        let results = db
            .search(
                "Hello",
                ContentType::All,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("test_video"),
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(
            results.len(),
            2,
            "Should find both frames in All content search"
        );

        // Count results with frame_name filter
        let count = db
            .count_search_results(
                "Hello",
                ContentType::OCR,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(count, 2, "Should count both matching frames");
    }

    #[tokio::test]
    async fn test_insert_and_search_ui_monitoring() {
        let db = setup_test_db().await;

        // Insert UI monitoring data
        sqlx::query(
            r#"
            INSERT INTO ui_monitoring (
                text_output,
                timestamp,
                app,
                window,
                initial_traversal_at
            ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind("Hello from UI monitoring")
        .bind(Utc::now())
        .bind("test_app")
        .bind("test_window")
        .bind(Utc::now())
        .execute(&db.pool)
        .await
        .unwrap();

        // Test search with app name filter
        let results = db
            .search(
                "Hello",
                ContentType::UI,
                100,
                0,
                None,
                None,
                Some("test_app"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        if let SearchResult::UI(ui_result) = &results[0] {
            assert_eq!(ui_result.text, "Hello from UI monitoring");
            assert_eq!(ui_result.app_name, "test_app");
            assert_eq!(ui_result.window_name, "test_window");
        } else {
            panic!("Expected UI result");
        }

        // Test search with window name filter
        let results = db
            .search(
                "Hello",
                ContentType::UI,
                100,
                0,
                None,
                None,
                None,
                Some("test_window"),
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 1);

        // Test search with no matches
        let results = db
            .search(
                "nonexistent",
                ContentType::UI,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 0);

        // Test search with empty query (should return all UI entries)
        let results = db
            .search(
                "",
                ContentType::UI,
                100,
                0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_count_search_results_all_content_types() {
        let db = setup_test_db().await;

        // Insert OCR data
        let _ = db
            .insert_video_chunk("test_video.mp4", "test_device")
            .await
            .unwrap();
        let frame_id = db
            .insert_frame("test_device", None, None, Some("test"), Some(""), false)
            .await
            .unwrap();
        db.insert_ocr_text(
            frame_id,
            "Hello from OCR",
            "",
            Arc::new(OcrEngine::Tesseract),
        )
        .await
        .unwrap();

        // Insert Audio data
        let audio_chunk_id = db.insert_audio_chunk("test_audio.mp4").await.unwrap();
        db.insert_audio_transcription(
            audio_chunk_id,
            "Hello from audio",
            0,
            "",
            &AudioDevice {
                name: "test".to_string(),
                device_type: DeviceType::Output,
            },
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // Insert UI monitoring data
        sqlx::query(
            r#"
            INSERT INTO ui_monitoring (
                text_output,
                timestamp,
                app,
                window,
                initial_traversal_at
            ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind("Hello from UI")
        .bind(Utc::now())
        .bind("test_app")
        .bind("test_window")
        .bind(Utc::now())
        .execute(&db.pool)
        .await
        .unwrap();

        // Test count with All content types
        let count = db
            .count_search_results(
                "Hello",
                ContentType::All,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(count, 3, "Should count OCR, Audio, and UI results");

        // Test count with specific app filter
        let count = db
            .count_search_results(
                "Hello",
                ContentType::All,
                None,
                None,
                Some("test_app"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(count, 1, "Should only count UI result with app filter");

        // Test count with non-matching query
        let count = db
            .count_search_results(
                "nonexistent",
                ContentType::All,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
        assert_eq!(count, 0, "Should count zero results for non-matching query");
    }
}
