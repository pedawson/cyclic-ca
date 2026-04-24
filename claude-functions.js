const {
  Document, Packer, Paragraph, TextRun, Table, TableRow, TableCell,
  AlignmentType, HeadingLevel, BorderStyle, WidthType, ShadingType,
  LevelFormat, Header, Footer, PageNumber
} = require('docx');
const fs = require('fs');

const border = { style: BorderStyle.SINGLE, size: 1, color: "DDDDDD" };
const borders = { top: border, bottom: border, left: border, right: border };
const noBorder = { style: BorderStyle.NONE, size: 0, color: "FFFFFF" };
const noBorders = { top: noBorder, bottom: noBorder, left: noBorder, right: noBorder };

function heading1(text) {
  return new Paragraph({
    heading: HeadingLevel.HEADING_1,
    spacing: { before: 360, after: 160 },
    children: [new TextRun({ text, font: "Arial", size: 36, bold: true, color: "1A1A2E" })]
  });
}

function heading2(text) {
  return new Paragraph({
    heading: HeadingLevel.HEADING_2,
    spacing: { before: 240, after: 120 },
    children: [new TextRun({ text, font: "Arial", size: 28, bold: true, color: "16213E" })]
  });
}

function body(text) {
  return new Paragraph({
    spacing: { after: 120 },
    children: [new TextRun({ text, font: "Arial", size: 22, color: "333333" })]
  });
}

function bullet(text) {
  return new Paragraph({
    numbering: { reference: "bullets", level: 0 },
    spacing: { after: 80 },
    children: [new TextRun({ text, font: "Arial", size: 22, color: "333333" })]
  });
}

function spacer(pt = 120) {
  return new Paragraph({ spacing: { after: pt }, children: [] });
}

function featureCard(title, subtitle, bullets) {
  const cellShade = { fill: "F0F4FF", type: ShadingType.CLEAR };
  const headerShade = { fill: "1A1A2E", type: ShadingType.CLEAR };

  return new Table({
    width: { size: 9360, type: WidthType.DXA },
    columnWidths: [9360],
    margins: { top: 0, bottom: 200 },
    rows: [
      new TableRow({
        children: [new TableCell({
          borders,
          width: { size: 9360, type: WidthType.DXA },
          shading: headerShade,
          margins: { top: 120, bottom: 120, left: 180, right: 180 },
          children: [
            new Paragraph({
              children: [
                new TextRun({ text: title, font: "Arial", size: 28, bold: true, color: "FFFFFF" }),
                new TextRun({ text: "  —  " + subtitle, font: "Arial", size: 22, color: "AABBDD" }),
              ]
            })
          ]
        })]
      }),
      new TableRow({
        children: [new TableCell({
          borders,
          width: { size: 9360, type: WidthType.DXA },
          shading: cellShade,
          margins: { top: 120, bottom: 140, left: 180, right: 180 },
          children: bullets.map(b => new Paragraph({
            numbering: { reference: "bullets", level: 0 },
            spacing: { after: 80 },
            children: [new TextRun({ text: b, font: "Arial", size: 22, color: "333333" })]
          }))
        })]
      })
    ]
  });
}

const doc = new Document({
  numbering: {
    config: [{
      reference: "bullets",
      levels: [{
        level: 0,
        format: LevelFormat.BULLET,
        text: "\u2022",
        alignment: AlignmentType.LEFT,
        style: { paragraph: { indent: { left: 540, hanging: 300 } } }
      }]
    }]
  },
  styles: {
    default: { document: { run: { font: "Arial", size: 22 } } },
    paragraphStyles: [
      {
        id: "Heading1", name: "Heading 1", basedOn: "Normal", next: "Normal", quickFormat: true,
        run: { size: 36, bold: true, font: "Arial", color: "1A1A2E" },
        paragraph: { spacing: { before: 360, after: 160 }, outlineLevel: 0 }
      },
      {
        id: "Heading2", name: "Heading 2", basedOn: "Normal", next: "Normal", quickFormat: true,
        run: { size: 28, bold: true, font: "Arial", color: "16213E" },
        paragraph: { spacing: { before: 240, after: 120 }, outlineLevel: 1 }
      }
    ]
  },
  sections: [{
    properties: {
      page: {
        size: { width: 12240, height: 15840 },
        margin: { top: 1440, right: 1440, bottom: 1440, left: 1440 }
      }
    },
    headers: {
      default: new Header({
        children: [new Paragraph({
          border: { bottom: { style: BorderStyle.SINGLE, size: 4, color: "1A1A2E", space: 1 } },
          children: [
            new TextRun({ text: "Claude — Major Functions Overview", font: "Arial", size: 18, color: "555555" })
          ]
        })]
      })
    },
    footers: {
      default: new Footer({
        children: [new Paragraph({
          alignment: AlignmentType.RIGHT,
          border: { top: { style: BorderStyle.SINGLE, size: 4, color: "1A1A2E", space: 1 } },
          children: [
            new TextRun({ text: "Page ", font: "Arial", size: 18, color: "888888" }),
            new TextRun({ children: [PageNumber.CURRENT], font: "Arial", size: 18, color: "888888" }),
            new TextRun({ text: " of ", font: "Arial", size: 18, color: "888888" }),
            new TextRun({ children: [PageNumber.TOTAL_PAGES], font: "Arial", size: 18, color: "888888" }),
          ]
        })]
      })
    },
    children: [
      // Title block
      new Paragraph({
        alignment: AlignmentType.CENTER,
        spacing: { before: 480, after: 80 },
        children: [new TextRun({ text: "Claude", font: "Arial", size: 72, bold: true, color: "1A1A2E" })]
      }),
      new Paragraph({
        alignment: AlignmentType.CENTER,
        spacing: { after: 480 },
        children: [new TextRun({ text: "Major Functions Overview", font: "Arial", size: 32, color: "4A4A8A", italics: true })]
      }),

      // Intro
      body("Claude is an AI assistant built by Anthropic. It operates across multiple surfaces and use cases — from natural conversation to agentic software development. Below is a summary of its major functional areas."),
      spacer(240),

      // Features
      featureCard("Chat", "Natural language conversation", [
        "Answer questions, explain concepts, summarize content",
        "Multi-turn conversations with persistent context",
        "Tone adapts to context: casual, professional, technical",
        "Available via claude.ai, API, and Claude Code",
      ]),
      spacer(160),

      featureCard("Code", "Software engineering assistance", [
        "Write, debug, refactor, and explain code in any language",
        "Claude Code CLI: agentic coding directly in your terminal",
        "IDE integrations: VS Code, JetBrains",
        "Reads files, runs commands, edits codebases end-to-end",
        "Supports multi-file projects, tests, and build pipelines",
      ]),
      spacer(160),

      featureCard("Review", "Code and document review", [
        "Review pull requests for bugs, security, and performance",
        "Provide inline comments and suggested changes",
        "Check for OWASP top-10 vulnerabilities",
        "Review documents, plans, and written content",
        "Structured feedback with actionable recommendations",
      ]),
      spacer(160),

      featureCard("Research & Analysis", "Deep investigation and synthesis", [
        "Web search and page crawling for up-to-date information",
        "Analyze PDFs, spreadsheets, images, and documents",
        "Synthesize findings across multiple sources",
        "Data extraction, comparison tables, and summaries",
        "Used for competitive research, incident analysis, and due diligence",
      ]),
      spacer(160),

      featureCard("Design", "Visual and document creation", [
        "Generate and edit designs via Canva MCP integration",
        "Create presentations, posters, and branded materials",
        "Produce Word (.docx), PDF, and PowerPoint (.pptx) documents",
        "Apply brand kits and maintain visual consistency",
        "Export to multiple formats",
      ]),
      spacer(160),

      featureCard("Cowork", "Collaborative multi-agent sessions", [
        "Shared sessions where multiple agents work in parallel",
        "Plugin ecosystem extending Claude with specialized tools",
        "Skills system for domain-specific workflows (debugging, TDD, etc.)",
        "Agents can spawn sub-agents for independent parallel tasks",
        "Works alongside humans in real-time iterative workflows",
      ]),
      spacer(160),

      featureCard("Email & Calendar", "Communication management", [
        "Read, search, and summarize Gmail threads",
        "Draft and send emails on your behalf",
        "Create, update, and respond to calendar events",
        "Daily digest summaries with priority triage",
        "Integrates via MCP with Google Workspace",
      ]),
      spacer(160),

      featureCard("Automation & Scheduling", "Recurring agents and workflows", [
        "Schedule recurring tasks via cron-style triggers",
        "Monitor builds, deploys, and background processes",
        "Set up hooks that fire on Claude Code events",
        "Run standup summaries, PR checks, or stock alerts automatically",
        "Chain multi-step workflows with conditional logic",
      ]),
      spacer(320),

      // Footer note
      new Paragraph({
        alignment: AlignmentType.CENTER,
        children: [new TextRun({ text: "anthropic.com  \u2022  claude.ai  \u2022  claude.ai/code", font: "Arial", size: 18, color: "888888" })]
      }),
    ]
  }]
});

Packer.toBuffer(doc).then(buf => {
  fs.writeFileSync("/Users/pauldawson/Desktop/Claude-Functions-Overview.docx", buf);
  console.log("Written: Claude-Functions-Overview.docx");
});
