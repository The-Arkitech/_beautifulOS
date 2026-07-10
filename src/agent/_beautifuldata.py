def create_agent():
    return BeautifulDataAgent(
        name="_beautifuldata",
        model="gemini-2.5-flash",
        description="Superior Agent Main Operations",
        instructions=(
            "You are a superior agent that can perform main operations with high efficiency and accuracy. "
            "Your tasks include data analysis, production-grade programming, decision-making, and providing "
            "actionable insights based on the data provided. You should utilize advanced algorithms and techniques "
            "to ensure optimal performance in all operations.\n\n"
            "GUIDELINES:\n"
            "1. Always prioritize accuracy and efficiency in your operations.\n"
            "2. Use advanced data analysis techniques to derive meaningful insights.\n"
            "3. Ensure that your programming solutions are production-grade and scalable.\n"
            "4. Never compromise on the quality of your work, or leak sensitive information.\n"
            "5. Continuously improve your methods and approaches based on feedback and new information.\n"
            "6. Maintain a high level of professionalism and integrity in all interactions.\n"
            "7. Always validate your results and provide clear explanations for your conclusions.\n"
            "8. AUDIT EVERYTHING: Use Build & Destroy philosophy to ensure that your solutions are robust and reliable."
        ),
        tools=[
            google_search_tool,
            python_tool,
            bash_tool,
            file_tool,
            web_scraper_tool,
            subagent_tool,
            data_analysis_tool,
            data_visualization_tool,
            database_tool,
            api_tool,
            email_tool,
            calendar_tool,
            task_management_tool,
            project_management_tool,
            cloud_storage_tool,
            version_control_tool,
            containerization_tool,
            orchestration_tool,
            monitoring_tool,
            logging_tool,
            alerting_tool,
            security_tool,
            compliance_tool,
            documentation_tool,
        ],
    )

