use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Resume {
    name: String,
    title: String,
    email: String,
    phone: String,
    website: Option<String>,
    location: Option<String>,
    linkedin: Option<String>,
    github: Option<String>,
    summary: Option<String>,
    experience: Vec<Experience>,
    education: Vec<Education>,
    skills: Option<Skills>,
    certifications: Option<Vec<Certification>>,
    projects: Option<Vec<Project>>,
    awards: Option<Vec<Award>>,
    publications: Option<Vec<Publication>>,
    languages: Option<Vec<Language>>,
    references: Option<Vec<Reference>>,
    speaking: Option<Vec<Speaking>>,
}

#[derive(Debug, Deserialize)]
struct Experience {
    company: String,
    role: String,
    location: Option<String>,
    start_date: String,
    end_date: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Education {
    institution: String,
    degree: String,
    location: Option<String>,
    start_date: String,
    end_date: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Skills {
    programming: Option<Vec<String>>,
    technical: Option<Vec<String>>,
    languages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Certification {
    name: String,
    issuer: String,
    date: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    date: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Award {
    title: String,
    issuer: String,
    date: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Publication {
    title: String,
    publisher: String,
    date: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Language {
    name: String,
    proficiency: String,
}

#[derive(Debug, Deserialize)]
struct Reference {
    name: String,
    relationship: String,
    email: String,
    phone: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Speaking {
    event: String,
    organization: String,
    date: String,
    location: String,
    description: String,
}

fn render_html(resume: &Resume) -> String {
    let mut html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{0} - Resume</title>
    <style>
        :root {{
            --primary-color: #2c3e50;
            --secondary-color: #7f8c8d;
            --accent-color: #3498db;
            --background-color: #f9f9f9;
            --border-color: #e0e0e0;
        }}
        
        body {{ 
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
            line-height: 1.6; 
            margin: 0; 
            padding: 20px; 
            color: #333; 
            background-color: var(--background-color);
        }}
        
        .container {{ 
            max-width: 800px; 
            margin: 0 auto; 
            background: white;
            padding: 30px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            border-radius: 5px;
        }}
        
        .header {{ 
            text-align: center; 
            margin-bottom: 30px; 
            padding-bottom: 20px;
            border-bottom: 2px solid var(--border-color);
        }}
        
        .header h1 {{ 
            margin: 0; 
            font-size: 32px; 
            color: var(--primary-color);
        }}
        
        .header h2 {{ 
            margin: 5px 0 15px; 
            font-size: 20px; 
            font-weight: normal; 
            color: var(--secondary-color);
        }}
        
        .contact-info {{ 
            margin: 15px 0; 
            display: flex;
            flex-wrap: wrap;
            justify-content: center;
            gap: 15px;
        }}
        
        .contact-info span {{ 
            display: inline-flex;
            align-items: center;
            gap: 5px;
        }}
        
        .section {{ 
            margin: 25px 0; 
        }}
        
        .section h3 {{ 
            border-bottom: 2px solid var(--accent-color); 
            padding-bottom: 8px; 
            color: var(--primary-color);
            margin-top: 0;
        }}
        
        .experience-item, .education-item {{ 
            margin: 18px 0; 
        }}
        
        .item-header {{
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            flex-wrap: wrap;
            gap: 10px;
        }}
        
        .job-title {{ 
            font-weight: bold; 
            font-size: 18px;
            color: var(--primary-color);
        }}
        
        .company {{ 
            font-style: italic; 
            color: var(--secondary-color);
        }}
        
        .date {{ 
            color: var(--secondary-color); 
            white-space: nowrap;
        }}
        
        .description {{
            margin-top: 8px;
        }}
        
        .skills-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 15px;
        }}
        
        .skill-category h4 {{
            margin: 0 0 8px 0;
            color: var(--primary-color);
        }}
        
        .skill-list {{
            list-style-type: none;
            padding: 0;
            margin: 0;
        }}
        
        .skill-list li {{
            padding: 3px 0;
        }}
        
        .two-column {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
        }}
        
        @media (max-width: 768px) {{
            .two-column {{
                grid-template-columns: 1fr;
            }}
            
            .contact-info {{
                flex-direction: column;
                align-items: center;
                gap: 8px;
            }}
            
            .item-header {{
                flex-direction: column;
                align-items: flex-start;
                gap: 5px;
            }}
        }}
        
        @media print {{
            body {{ 
                padding: 0; 
                background: white;
            }}
            
            .container {{ 
                max-width: 100%; 
                box-shadow: none;
                padding: 0;
            }}
            
            .contact-info {{
                justify-content: flex-start;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{0}</h1>
            <h2>{1}</h2>
            <div class="contact-info">
                <span>📧 {2}</span>
                <span>📞 {3}</span>"#,
        resume.name, resume.title, resume.email, resume.phone
    );

    // Add optional contact fields
    if let Some(website) = &resume.website {
        html.push_str(&format!(r#"<span>🌐 {}</span>"#, website));
    }
    if let Some(location) = &resume.location {
        html.push_str(&format!(r#"<span>📍 {}</span>"#, location));
    }
    if let Some(linkedin) = &resume.linkedin {
        html.push_str(&format!(r#"<span>💼 {}</span>"#, linkedin));
    }
    if let Some(github) = &resume.github {
        html.push_str(&format!(r#"<span>🐙 {}</span>"#, github));
    }

    html.push_str(r#"</div></div>"#);

    // Summary section
    if let Some(summary) = &resume.summary {
        html.push_str(&format!(
            r#"<div class="section"><h3>Professional Summary</h3><p>{}</p></div>"#,
            summary.replace("\n", "<br>")
        ));
    }

    // Experience section
    if !resume.experience.is_empty() {
        html.push_str(r#"<div class="section"><h3>Work Experience</h3>"#);
        for job in &resume.experience {
            html.push_str(&format!(
                r#"<div class="experience-item">
                    <div class="item-header">
                        <div class="job-title">{}</div>
                        <div class="date">{} - {}</div>
                    </div>
                    <div class="company">{}{}</div>
                    <div class="description">{}</div>
                </div>"#,
                job.role,
                job.start_date,
                job.end_date,
                job.company,
                job.location
                    .as_ref()
                    .map(|loc| format!(", {}", loc))
                    .unwrap_or_default(),
                job.description.replace("\n", "<br>")
            ));
        }
        html.push_str("</div>");
    }

    // Education section
    if !resume.education.is_empty() {
        html.push_str(r#"<div class="section"><h3>Education</h3>"#);
        for edu in &resume.education {
            html.push_str(&format!(
                r#"<div class="education-item">
                    <div class="item-header">
                        <div class="job-title">{}</div>
                        <div class="date">{} - {}</div>
                    </div>
                    <div class="company">{}{}</div>
                    {}
                </div>"#,
                edu.degree,
                edu.start_date,
                edu.end_date,
                edu.institution,
                edu.location
                    .as_ref()
                    .map(|loc| format!(", {}", loc))
                    .unwrap_or_default(),
                edu.description
                    .as_ref()
                    .map(|desc| format!("<div class=\"description\">{}</div>", desc.replace("\n", "<br>")))
                    .unwrap_or_default()
            ));
        }
        html.push_str("</div>");
    }

    // Skills section
    if let Some(skills) = &resume.skills {
        html.push_str(r#"<div class="section"><h3>Skills</h3><div class="skills-grid">"#);
        
        if let Some(programming) = &skills.programming {
            if !programming.is_empty() {
                html.push_str(r#"<div class="skill-category"><h4>Programming</h4><ul class="skill-list">"#);
                for skill in programming {
                    html.push_str(&format!("<li>{}</li>", skill));
                }
                html.push_str("</ul></div>");
            }
        }
        
        if let Some(technical) = &skills.technical {
            if !technical.is_empty() {
                html.push_str(r#"<div class="skill-category"><h4>Technical</h4><ul class="skill-list">"#);
                for skill in technical {
                    html.push_str(&format!("<li>{}</li>", skill));
                }
                html.push_str("</ul></div>");
            }
        }
        
        if let Some(languages) = &skills.languages {
            if !languages.is_empty() {
                html.push_str(r#"<div class="skill-category"><h4>Languages</h4><ul class="skill-list">"#);
                for skill in languages {
                    html.push_str(&format!("<li>{}</li>", skill));
                }
                html.push_str("</ul></div>");
            }
        }
        
        html.push_str("</div></div>");
    }

    // Projects section
    if let Some(projects) = &resume.projects {
        if !projects.is_empty() {
            html.push_str(r#"<div class="section"><h3>Projects</h3>"#);
            for project in projects {
                html.push_str(&format!(
                    r#"<div class="experience-item">
                        <div class="item-header">
                            <div class="job-title">{}</div>
                            <div class="date">{}</div>
                        </div>
                        <div class="description">{}</div>
                    </div>"#,
                    project.name,
                    project.date,
                    project.description.replace("\n", "<br>")
                ));
            }
            html.push_str("</div>");
        }
    }

    // Certifications section
    if let Some(certifications) = &resume.certifications {
        if !certifications.is_empty() {
            html.push_str(r#"<div class="section"><h3>Certifications</h3>"#);
            for cert in certifications {
                html.push_str(&format!(
                    r#"<div class="experience-item">
                        <div class="item-header">
                            <div class="job-title">{}</div>
                            <div class="date">{}</div>
                        </div>
                        <div class="company">{}</div>
                        {}
                    </div>"#,
                    cert.name,
                    cert.date,
                    cert.issuer,
                    cert.description
                        .as_ref()
                        .map(|desc| format!("<div class=\"description\">{}</div>", desc))
                        .unwrap_or_default()
                ));
            }
            html.push_str("</div>");
        }
    }

    // Awards section
    if let Some(awards) = &resume.awards {
        if !awards.is_empty() {
            html.push_str(r#"<div class="section"><h3>Awards & Honors</h3>"#);
            for award in awards {
                html.push_str(&format!(
                    r#"<div class="experience-item">
                        <div class="item-header">
                            <div class="job-title">{}</div>
                            <div class="date">{}</div>
                        </div>
                        <div class="company">{}</div>
                        {}
                    </div>"#,
                    award.title,
                    award.date,
                    award.issuer,
                    award.description
                        .as_ref()
                        .map(|desc| format!("<div class=\"description\">{}</div>", desc))
                        .unwrap_or_default()
                ));
            }
            html.push_str("</div>");
        }
    }

    // Publications section
    if let Some(publications) = &resume.publications {
        if !publications.is_empty() {
            html.push_str(r#"<div class="section"><h3>Publications</h3>"#);
            for pub_item in publications {
                html.push_str(&format!(
                    r#"<div class="experience-item">
                        <div class="item-header">
                            <div class="job-title">{}</div>
                            <div class="date">{}</div>
                        </div>
                        <div class="company">{}</div>
                        <div class="description">{}</div>
                    </div>"#,
                    pub_item.title,
                    pub_item.date,
                    pub_item.publisher,
                    pub_item.description.replace("\n", "<br>")
                ));
            }
            html.push_str("</div>");
        }
    }

    // Languages section
    if let Some(languages) = &resume.languages {
        if !languages.is_empty() {
            html.push_str(r#"<div class="section"><h3>Languages</h3><ul class="skill-list">"#);
            for lang in languages {
                html.push_str(&format!(
                    "<li><strong>{}:</strong> {}</li>",
                    lang.name, lang.proficiency
                ));
            }
            html.push_str("</ul></div>");
        }
    }

    // References section
    if let Some(references) = &resume.references {
        if !references.is_empty() {
            html.push_str(r#"<div class="section"><h3>References</h3>"#);
            for reference in references {
                html.push_str(&format!(
                    r#"<div class="experience-item">
                        <div class="job-title">{}</div>
                        <div class="company">{}</div>
                        <div>📧 {} {}</div>
                        {}
                    </div>"#,
                    reference.name,
                    reference.relationship,
                    reference.email,
                    reference.phone
                        .as_ref()
                        .map(|phone| format!("📞 {}", phone))
                        .unwrap_or_default(),
                    reference.description
                        .as_ref()
                        .map(|desc| format!("<div class=\"description\">{}</div>", desc))
                        .unwrap_or_default()
                ));
            }
            html.push_str("</div>");
        }
    }

    // Speaking engagements section
    if let Some(speaking) = &resume.speaking {
        if !speaking.is_empty() {
            html.push_str(r#"<div class="section"><h3>Speaking Engagements</h3>"#);
            for talk in speaking {
                html.push_str(&format!(
                    r#"<div class="experience-item">
                        <div class="item-header">
                            <div class="job-title">{}</div>
                            <div class="date">{}</div>
                        </div>
                        <div class="company">{}, {}</div>
                        <div class="description">{}</div>
                    </div>"#,
                    talk.event,
                    talk.date,
                    talk.organization,
                    talk.location,
                    talk.description.replace("\n", "<br>")
                ));
            }
            html.push_str("</div>");
        }
    }

    html.push_str(r#"</div></body></html>"#);
    html
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read and parse TOML
    let toml_content = fs::read_to_string("resume.toml")
        .map_err(|e| format!("Failed to read TOML file: {}", e))?;
    
    let resume: Resume = toml::from_str(&toml_content)
        .map_err(|e| format!("Failed to parse TOML: {}", e))?;

    // Generate HTML
    let html = render_html(&resume);
    fs::write("resume.html", &html)
        .map_err(|e| format!("Failed to write HTML file: {}", e))?;

    println!("✅ Resume HTML generated: resume.html");

    // Try to generate PDF with WeasyPrint
    let weasyprint_result = Command::new("weasyprint")
        .args(["resume.html", "resume.pdf"])
        .status();
    
    match weasyprint_result {
        Ok(status) if status.success() => {
            println!("📄 PDF generated with WeasyPrint: resume.pdf");
            Ok(())
        },
        Ok(_) => {
            eprintln!("⚠️ WeasyPrint failed to generate PDF");
            Ok(())
        },
        Err(_) => {
            eprintln!("⚠️ WeasyPrint not available. Install it with:");
            eprintln!("  macOS: brew install weasyprint");
            eprintln!("  Ubuntu/Debian: sudo apt-get install weasyprint");
            eprintln!("  Windows: pip install weasyprint");
            Ok(())
        }
    }
}