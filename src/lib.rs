//! # ternary-form
//!
//! Musical form analysis for multi-agent task decomposition. In music, form is the
//! large-scale structure — how sections relate to each other across time. This crate
//! applies musical form concepts to multi-agent task planning and analysis, providing
//! structural templates and detection tools for common patterns.


/// A section within a musical form — a distinct phase of a task.
#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    /// Label for this section (e.g., "A", "B", "development").
    pub label: String,
    /// Human-readable description.
    pub description: String,
    /// Which agent is responsible (if assigned).
    pub agent: Option<String>,
    /// Estimated relative duration (1.0 = normal).
    pub weight: f64,
}

impl Section {
    /// Create a new section.
    pub fn new(label: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: description.into(),
            agent: None,
            weight: 1.0,
        }
    }

    /// Assign an agent to this section.
    pub fn with_agent(mut self, agent: impl Into<String>) -> Self {
        self.agent = Some(agent.into());
        self
    }

    /// Set the weight (relative duration).
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }
}

/// A complete musical form — a sequence of sections.
#[derive(Debug, Clone)]
pub struct Form {
    /// Name of this form.
    pub name: String,
    /// Ordered sections.
    pub sections: Vec<Section>,
}

impl Form {
    /// Create a new form with a name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sections: Vec::new(),
        }
    }

    /// Add a section to the form.
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// Get the section labels as a pattern string (e.g., "A-B-A").
    pub fn pattern(&self) -> String {
        self.sections
            .iter()
            .map(|s| s.label.as_str())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Get the unique section labels.
    pub fn unique_labels(&self) -> Vec<&str> {
        let mut seen = vec![];
        for s in &self.sections {
            if !seen.contains(&s.label.as_str()) {
                seen.push(s.label.as_str());
            }
        }
        seen
    }

    /// Total weight (estimated total duration).
    pub fn total_weight(&self) -> f64 {
        self.sections.iter().map(|s| s.weight).sum()
    }
}

/// Binary form: A-B — two contrasting sections.
///
/// In music: a composition with two distinct parts, often with contrasting character.
/// In tasks: a two-phase approach where the second phase contrasts with the first
/// (e.g., research → implementation, explore → exploit).
pub struct BinaryForm;

impl BinaryForm {
    /// Build a binary form (A-B) from two section descriptions.
    pub fn build(
        section_a: impl Into<String>,
        section_b: impl Into<String>,
    ) -> Form {
        let mut form = Form::new("Binary");
        form.add_section(Section::new("A", section_a));
        form.add_section(Section::new("B", section_b));
        form
    }

    /// Check if a form follows binary (A-B) structure.
    pub fn matches(form: &Form) -> bool {
        if form.sections.len() != 2 {
            return false;
        }
        form.sections[0].label != form.sections[1].label
    }
}

/// Ternary form: A-B-A — departure and return.
///
/// In music: the most common form — a theme, a contrasting middle, then return.
/// In tasks: establish context → explore alternative → return with insight.
/// The return is never quite the same as the opening — it's informed by the middle.
pub struct TernaryForm;

impl TernaryForm {
    /// Build a ternary form (A-B-A) from three section descriptions.
    pub fn build(
        section_a: impl Into<String>,
        section_b: impl Into<String>,
        section_a_prime: impl Into<String>,
    ) -> Form {
        let mut form = Form::new("Ternary");
        form.add_section(Section::new("A", section_a));
        form.add_section(Section::new("B", section_b));
        form.add_section(Section::new("A'", section_a_prime));
        form
    }

    /// Check if a form follows ternary (A-B-A) structure.
    pub fn matches(form: &Form) -> bool {
        if form.sections.len() != 3 {
            return false;
        }
        // First and third should be related (A and A'), middle is different (B)
        let first = &form.sections[0].label;
        let second = &form.sections[1].label;
        let third = &form.sections[2].label;

        // A' starts with A or is same
        let returns = third.starts_with(first) || first.starts_with(third);
        let contrasts = first != second && second != third;

        returns && contrasts
    }
}

/// Rondo form: A-B-A-C-A — recurring theme with contrasting episodes.
///
/// In music: a principal theme alternates with contrasting episodes.
/// In tasks: a core objective keeps returning between digressions.
/// Perfect for iterative refinement with exploratory phases.
pub struct RondoForm;

impl RondoForm {
    /// Build a rondo form with a recurring theme and episodes.
    pub fn build(theme: impl Into<String>, episodes: Vec<impl Into<String>>) -> Form {
        let mut form = Form::new("Rondo");
        let theme_str = theme.into();

        form.add_section(Section::new("A", theme_str.clone()));

        for (i, episode) in episodes.into_iter().enumerate() {
            let label = match i {
                0 => "B",
                1 => "C",
                2 => "D",
                3 => "E",
                _ => "X",
            };
            form.add_section(Section::new(label, episode));
            form.add_section(Section::new("A", format!("{} (return #{})", theme_str, i + 1)));
        }

        form
    }

    /// Check if a form follows rondo structure (A recurring between contrasting sections).
    pub fn matches(form: &Form) -> bool {
        if form.sections.len() < 5 {
            return false;
        }

        // Check that A appears at positions 0, 2, 4, ...
        let positions_of_a: Vec<usize> = form
            .sections
            .iter()
            .enumerate()
            .filter(|(_, s)| s.label == "A")
            .map(|(i, _)| i)
            .collect();

        if positions_of_a.len() < 3 {
            return false;
        }

        // A should appear at even positions: 0, 2, 4, ...
        positions_of_a.iter().all(|&p| p % 2 == 0)
    }
}

/// Sonata form: exposition → development → recapitulation.
///
/// The most sophisticated musical form. Three major sections:
/// 1. **Exposition**: Present two contrasting themes (tonic and dominant keys)
/// 2. **Development**: Fragment, combine, and transform the themes
/// 3. **Recapitulation**: Restate both themes, now reconciled (both in tonic)
///
/// In tasks: present the problem from two angles, explore the tension, resolve.
pub struct SonataForm;

impl SonataForm {
    /// Build a sonata form from descriptions of each section.
    pub fn build(
        exposition_theme_1: impl Into<String>,
        exposition_theme_2: impl Into<String>,
        development: impl Into<String>,
        recap_theme_1: impl Into<String>,
        recap_theme_2: impl Into<String>,
    ) -> Form {
        let mut form = Form::new("Sonata");

        // Exposition
        form.add_section(Section::new("Exp-1", exposition_theme_1).with_weight(1.0));
        form.add_section(Section::new("Exp-2", exposition_theme_2).with_weight(1.0));

        // Development
        form.add_section(Section::new("Dev", development).with_weight(1.5));

        // Recapitulation
        form.add_section(
            Section::new("Recap-1", recap_theme_1).with_weight(1.0),
        );
        form.add_section(
            Section::new("Recap-2", recap_theme_2).with_weight(1.0),
        );

        form
    }

    /// Check if a form follows sonata structure.
    pub fn matches(form: &Form) -> bool {
        if form.sections.len() != 5 {
            return false;
        }

        let labels: Vec<&str> = form.sections.iter().map(|s| s.label.as_str()).collect();

        // Check for exposition-development-recapitulation pattern
        let has_exp = labels[0].starts_with("Exp") && labels[1].starts_with("Exp");
        let has_dev = labels[2].starts_with("Dev");
        let has_recap = labels[3].starts_with("Recap") && labels[4].starts_with("Recap");

        has_exp && has_dev && has_recap
    }
}

/// Identifies the form of a task sequence by analyzing section patterns.
pub struct FormDetector;

impl FormDetector {
    /// Detect the form of a given task sequence.
    pub fn detect(form: &Form) -> Option<&'static str> {
        if BinaryForm::matches(form) {
            return Some("Binary");
        }
        if TernaryForm::matches(form) {
            return Some("Ternary");
        }
        if SonataForm::matches(form) {
            return Some("Sonata");
        }
        if RondoForm::matches(form) {
            return Some("Rondo");
        }
        None
    }

    /// Detect form with confidence score.
    pub fn detect_with_confidence(form: &Form) -> Option<(&'static str, f64)> {
        match Self::detect(form) {
            Some("Binary") => Some(("Binary", 0.95)),
            Some("Ternary") => Some(("Ternary", 0.9)),
            Some("Sonata") => Some(("Sonata", 0.95)),
            Some("Rondo") => Some(("Rondo", 0.85)),
            Some(name) => Some((name, 0.5)),
            None => None,
        }
    }
}

/// Builds task plans following specific musical forms.
pub struct FormBuilder;

impl FormBuilder {
    /// Build a binary form task plan with agents.
    pub fn binary(
        agent_a: impl Into<String>,
        agent_b: impl Into<String>,
        task_a: impl Into<String>,
        task_b: impl Into<String>,
    ) -> Form {
        let mut form = BinaryForm::build(task_a, task_b);
        form.sections[0] = form.sections[0].clone().with_agent(agent_a);
        form.sections[1] = form.sections[1].clone().with_agent(agent_b);
        form
    }

    /// Build a ternary form task plan with agents.
    pub fn ternary<S: Into<String>>(
        agent_a: S,
        agent_b: S,
        task_a: S,
        task_b: S,
        task_a_prime: S,
    ) -> Form {
        let agent_a_str = agent_a.into();
        let mut form = TernaryForm::build(
            task_a.into(),
            task_b.into(),
            task_a_prime.into(),
        );
        form.sections[0] = form.sections[0].clone().with_agent(agent_a_str.clone());
        form.sections[1] = form.sections[1].clone().with_agent(agent_b.into());
        form.sections[2] = form.sections[2].clone().with_agent(agent_a_str);
        form
    }

    /// Build a rondo form task plan.
    pub fn rondo<S1, S2>(
        theme_agent: S1,
        episode_agents: Vec<S1>,
        theme_task: S2,
        episode_tasks: Vec<S2>,
    ) -> Form
    where
        S1: Into<String> + Clone,
        S2: Into<String>,
    {
        let mut form = RondoForm::build(theme_task, episode_tasks);
        let theme_agent_str: String = theme_agent.into();

        // Assign theme agent to all A sections
        for section in &mut form.sections {
            if section.label == "A" {
                section.agent = Some(theme_agent_str.clone());
            }
        }

        // Assign episode agents to contrasting sections
        let episode_agents_str: Vec<String> = episode_agents.into_iter().map(|a| a.into()).collect();
        let mut ep_idx = 0;
        for section in &mut form.sections {
            if section.label != "A" && ep_idx < episode_agents_str.len() {
                section.agent = Some(episode_agents_str[ep_idx].clone());
                ep_idx += 1;
            }
        }

        form
    }

    /// Build a sonata form task plan.
    pub fn sonata(
        exp_agents: (impl Into<String>, impl Into<String>),
        dev_agent: impl Into<String>,
        recap_agents: (impl Into<String>, impl Into<String>),
        exp_tasks: (impl Into<String>, impl Into<String>),
        dev_task: impl Into<String>,
        recap_tasks: (impl Into<String>, impl Into<String>),
    ) -> Form {
        let mut form = SonataForm::build(
            exp_tasks.0,
            exp_tasks.1,
            dev_task,
            recap_tasks.0,
            recap_tasks.1,
        );

        form.sections[0] = form.sections[0].clone().with_agent(exp_agents.0);
        form.sections[1] = form.sections[1].clone().with_agent(exp_agents.1);
        form.sections[2] = form.sections[2].clone().with_agent(dev_agent);
        form.sections[3] = form.sections[3].clone().with_agent(recap_agents.0);
        form.sections[4] = form.sections[4].clone().with_agent(recap_agents.1);

        form
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_form_detection() {
        let form = BinaryForm::build("Research the problem", "Implement the solution");
        assert_eq!(form.pattern(), "A-B");
        assert!(BinaryForm::matches(&form));
        assert_eq!(FormDetector::detect(&form), Some("Binary"));
    }

    #[test]
    fn test_binary_form_doesnt_match_ternary() {
        let form = TernaryForm::build("Start", "Middle", "Return");
        assert!(!BinaryForm::matches(&form));
    }

    #[test]
    fn test_ternary_form_detection() {
        let form = TernaryForm::build(
            "Establish baseline metrics",
            "Explore alternative approaches",
            "Return to baseline with improvements",
        );
        assert_eq!(form.pattern(), "A-B-A'");
        assert!(TernaryForm::matches(&form));
        assert_eq!(FormDetector::detect(&form), Some("Ternary"));
    }

    #[test]
    fn test_ternary_form_requires_return() {
        // A-B-B is not ternary
        let mut form = Form::new("BadTernary");
        form.add_section(Section::new("A", "first"));
        form.add_section(Section::new("B", "second"));
        form.add_section(Section::new("B", "third"));
        assert!(!TernaryForm::matches(&form));
    }

    #[test]
    fn test_rondo_pattern() {
        let form = RondoForm::build(
            "Core analysis",
            vec!["Explore edge cases", "Check performance", "Review security"],
        );

        // Should be A-B-A-C-A-D-A
        assert_eq!(form.sections.len(), 7);
        assert_eq!(form.sections[0].label, "A");
        assert_eq!(form.sections[1].label, "B");
        assert_eq!(form.sections[2].label, "A");
        assert_eq!(form.sections[3].label, "C");
        assert_eq!(form.sections[4].label, "A");
        assert_eq!(form.sections[5].label, "D");
        assert_eq!(form.sections[6].label, "A");
    }

    #[test]
    fn test_rondo_detection() {
        let form = RondoForm::build("Theme", vec!["Ep1", "Ep2"]);
        assert!(RondoForm::matches(&form));
        assert_eq!(FormDetector::detect(&form), Some("Rondo"));
    }

    #[test]
    fn test_sonata_structure() {
        let form = SonataForm::build(
            "Define the problem scope",
            "Identify stakeholder concerns",
            "Deep dive into tensions between scope and concerns",
            "Restate problem with stakeholder alignment",
            "Present unified solution",
        );

        assert_eq!(form.sections.len(), 5);
        assert!(form.sections[0].label.starts_with("Exp"));
        assert!(form.sections[2].label.starts_with("Dev"));
        assert!(form.sections[3].label.starts_with("Recap"));
        assert!(SonataForm::matches(&form));
        assert_eq!(FormDetector::detect(&form), Some("Sonata"));
    }

    #[test]
    fn test_sonata_weight() {
        let form = SonataForm::build("a", "b", "c", "d", "e");
        // Development has weight 1.5, others 1.0
        assert_eq!(form.sections[2].weight, 1.5);
        assert_eq!(form.total_weight(), 5.5);
    }

    #[test]
    fn test_form_builder_binary() {
        let form = FormBuilder::binary(
            "researcher",
            "implementer",
            "Gather requirements",
            "Build the feature",
        );

        assert_eq!(form.sections[0].agent.as_deref(), Some("researcher"));
        assert_eq!(form.sections[1].agent.as_deref(), Some("implementer"));
        assert!(BinaryForm::matches(&form));
    }

    #[test]
    fn test_form_builder_ternary() {
        let form = FormBuilder::ternary(
            "analyst",
            "creative",
            "Baseline analysis",
            "Creative exploration",
            "Refined analysis",
        );

        // A sections should use analyst, B uses creative
        assert_eq!(form.sections[0].agent.as_deref(), Some("analyst"));
        assert_eq!(form.sections[1].agent.as_deref(), Some("creative"));
        assert_eq!(form.sections[2].agent.as_deref(), Some("analyst"));
    }

    #[test]
    fn test_form_builder_rondo() {
        let form = FormBuilder::rondo(
            "coordinator",
            vec!["explorer-a", "explorer-b"],
            "Review progress",
            vec!["Explore option A", "Explore option B"],
        );

        // All A sections should have coordinator
        for section in &form.sections {
            if section.label == "A" {
                assert_eq!(section.agent.as_deref(), Some("coordinator"));
            }
        }
        assert!(RondoForm::matches(&form));
    }

    #[test]
    fn test_form_builder_sonata() {
        let form = FormBuilder::sonata(
            ("agent-1", "agent-2"),
            "dev-agent",
            ("agent-3", "agent-4"),
            ("Present view A", "Present view B"),
            "Tension exploration",
            ("Synthesis A", "Synthesis B"),
        );

        assert_eq!(form.sections[0].agent.as_deref(), Some("agent-1"));
        assert_eq!(form.sections[2].agent.as_deref(), Some("dev-agent"));
        assert_eq!(form.sections[3].agent.as_deref(), Some("agent-3"));
        assert!(SonataForm::matches(&form));
    }

    #[test]
    fn test_form_pattern_string() {
        let mut form = Form::new("Test");
        form.add_section(Section::new("A", "first"));
        form.add_section(Section::new("B", "second"));
        form.add_section(Section::new("A", "third"));
        assert_eq!(form.pattern(), "A-B-A");
    }

    #[test]
    fn test_form_unique_labels() {
        let mut form = Form::new("Test");
        form.add_section(Section::new("A", "first"));
        form.add_section(Section::new("B", "second"));
        form.add_section(Section::new("A", "third"));
        form.add_section(Section::new("C", "fourth"));

        let labels = form.unique_labels();
        assert_eq!(labels, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_section_builder() {
        let section = Section::new("A", "Main theme")
            .with_agent("agent-1")
            .with_weight(2.0);

        assert_eq!(section.label, "A");
        assert_eq!(section.description, "Main theme");
        assert_eq!(section.agent.as_deref(), Some("agent-1"));
        assert_eq!(section.weight, 2.0);
    }

    #[test]
    fn test_detect_with_confidence() {
        let form = BinaryForm::build("A", "B");
        let result = FormDetector::detect_with_confidence(&form);
        assert_eq!(result, Some(("Binary", 0.95)));
    }

    #[test]
    fn test_detect_no_match() {
        let mut form = Form::new("Unknown");
        form.add_section(Section::new("X", "first"));
        form.add_section(Section::new("Y", "second"));
        form.add_section(Section::new("Z", "third"));
        form.add_section(Section::new("W", "fourth"));

        assert!(FormDetector::detect(&form).is_none());
    }

    #[test]
    fn test_detect_with_confidence_no_match() {
        let mut form = Form::new("Unknown");
        form.add_section(Section::new("X", "a"));
        form.add_section(Section::new("Y", "b"));
        form.add_section(Section::new("Z", "c"));
        form.add_section(Section::new("W", "d"));
        assert!(FormDetector::detect_with_confidence(&form).is_none());
    }

    #[test]
    fn test_rondo_too_short_not_rondo() {
        let form = RondoForm::build("Theme", vec!["Only one episode"]);
        assert!(!RondoForm::matches(&form), "Rondo needs at least 5 sections (2+ episodes)");
    }

    #[test]
    fn test_binary_wrong_length() {
        let mut form = Form::new("Test");
        form.add_section(Section::new("A", "only one"));
        assert!(!BinaryForm::matches(&form));

        form.add_section(Section::new("A", "same label"));
        assert!(!BinaryForm::matches(&form), "Both sections same label = not binary");
    }
}
