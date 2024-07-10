-- Your SQL goes here
CREATE TABLE plugins(
    name TEXT NOT NULL PRIMARY KEY,
    description TEXT NOT NULL,
    time timestamp NOT NULL,
    version TEXT NOT NULL,
    official BOOLEAN NOT NULL
);


CREATE TABLE datas(
   id TEXT NOT NULL PRIMARY KEY,
   plugin_name TEXT NOT NULL,
  _id TEXT NOT NULL,
  _rev TEXT NOT NULL,
  name TEXT NOT NULL,
  license TEXT,
  downloads INTEGER NOT NULL,
  FOREIGN KEY (plugin_name) REFERENCES plugins(name)
);

CREATE TABLE versions(
    id TEXT NOT NULL PRIMARY KEY,
    data_id TEXT NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT NOT NULL,
    time timestamp NOT NULL,
    author_name TEXT NOT NULL,
    author_email TEXT NOT NULL,
    license TEXT,
    repository_type TEXT,
    repository_url TEXT,
    keywords TEXT,
    image TEXT,
    readme TEXT,
    FOREIGN KEY (data_id) REFERENCES datas(id)
);

CREATE TABLE keywords(
    id TEXT NOT NULL PRIMARY KEY,
    version_id TEXT NOT NULL,
    keyword TEXT NOT NULL,
    FOREIGN KEY (version_id) REFERENCES versions(id)
);


CREATE TABLE officialRepositories(
    id TEXT NOT  NULL PRIMARY KEY
);

CREATE INDEX versions_name ON versions(name);
CREATE INDEX datas_name ON datas(name);


CREATE TABLE timestamp_sync(
       id TEXT NOT NULL PRIMARY KEY,
       timestamp Timestamp NOT NULL
);


CREATE TABLE sequences(
    id TEXT NOT NULL PRIMARY KEY,
    val BigInt NOT NULL
);


CREATE TABLE plugin_shorts(
    name TEXT NOT NULL PRIMARY KEY,
    description TEXT,
    time_downloaded TEXT,
    version TEXT NOT NULL,
    official BOOLEAN NOT NULL,
    downloads INTEGER NULL
);


INSERT OR IGNORE INTO sequences(id, val) VALUES('sequence', 11458940);

CREATE TABLE ep_changes(
  name TEXT NOT NULL PRIMARY KEY,
  seq_id BIGINT NOT NULL
);

INSERT INTO ep_changes (name, seq_id) VALUES ('ep_collabticker', 6482999);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_bootstrap', 6496551);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_linote_markdown', 7105196);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_comments_page22', 10287236);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_mobile_eruda', 10506355);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_catalog_plugin', 10628195);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_font_color2', 10636324);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_embeded_media', 10667029);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_font_color3', 10697356);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_no_console', 10728502);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_back_button2', 10731293);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_embedded_hyperlinks3', 10731294);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_image_upload2', 10732548);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_todo_list', 10732549);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_comments_page2', 10739718);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_tables5', 10739720);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_allmende', 10988889);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_apples', 10988891);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_audio_upload', 10988892);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_auth_session_relative', 10988897);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_auth_session_same_site', 10988898);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_auth_session_samesite_none', 10988899);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_auth_session_samesite_none_more_params', 10988900);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_authentication', 10988901);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_author_line', 10988906);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_author_neat2', 10988907);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_back_button', 10988923);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_bottom_chat_bar', 10988926);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_caret_trace', 10988928);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_catalog', 10988929);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_code_formatting', 10988939);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_comments_page_short_edition', 10988944);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_content_title', 10988945);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_convert', 10988948);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_convert_to_html', 10988950);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_custom_header_message', 10988955);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_database_query_auth', 10988959);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_delete_after_delay_lite', 10988965);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_demo', 10988967);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_dividing_line', 10988976);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_embedded_hyperlinks2', 10988984);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_etherpad-lite', 10988987);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_feedback', 10988991);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_font_size2', 10988997);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_font_size_citizenos', 10988998);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_full_hyperlinks', 10989002);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_h5_home', 10989007);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_heading_plugin', 10989012);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_headings_css', 10989014);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_health_check', 10989015);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_httpauth_author', 10989020);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_hyperwrite_dev', 10989022);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_ice_font_color', 10989027);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_ice_font_size', 10989028);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_ice_headings', 10989029);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_ice_hyperlinks', 10989030);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_ice_image_upload', 10989031);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_ice_insert', 10989033);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_ice_tables', 10989034);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_imageconvert', 10989036);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_inline_toolbar_pro', 10989042);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_insert_media', 10989044);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_jwt', 10989048);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_limit_maximum_number_of_users_on_pad', 10989051);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_line_spacing', 10989053);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_log_auth_failure_ip', 10989059);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_mention_plugin', 10989068);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_monetization', 10989074);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_my_example', 10989076);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_my_examples', 10989077);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_nomnoml', 10989081);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_npm_package', 10989082);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_oar', 10989083);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_oauth_mattermost', 10989085);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_openid-client', 10989087);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_pad_overview', 10989092);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_padwoman_button', 10989098);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_password_change', 10989101);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_plantuml', 10989104);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_pomerium', 10989105);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_private_pad', 10989110);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_profile_modal', 10989111);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_promclient', 10989112);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_remove_savedrevision_right_side', 10989122);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_share_plugin', 10989133);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_short_countable', 10989134);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_simple_creator', 10989137);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_social_preview', 10989141);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_solar', 10989143);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_syntax', 10989159);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_tables3', 10989166);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_tables4', 10989168);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_termin_app_integration', 10989196);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_timestamp', 10989206);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_title_limit', 10989207);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_two_editors', 10989210);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_user_fontsize_version', 10989212);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_user_fontsize_version_2', 10989213);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_video_upload', 10989214);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_visual_slider', 10989215);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_wags', 10989216);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_webpack', 10989221);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_whiteboard', 10989224);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_workspaces', 10989226);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_wrtc_heading', 10989228);
INSERT INTO ep_changes (name, seq_id) VALUES ('ep_zigzag', 10989229);
