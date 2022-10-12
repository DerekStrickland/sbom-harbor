"""
-> Test for the codebase handlers
"""
import uuid
from json import (
    dumps,
    loads,
)

import boto3
import pytest
from moto import mock_dynamodb

from cyclonedx.model.project import Project
from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.model.team import Team
from cyclonedx.model.codebase import CodeBase

# TODO I'm testing moving this here to see
#  if the @mock_dynamodb annotation still works.
#  Pylint hates imports inside of functions, so
#  we should try leaving it here. However, if this
#  test fails, be highly suspicious of this
#  and move it back into the test function.
#  The Pylint error can be suppressed with:
#  # pylint: disable=C0415
#  over the imports inside the test function.
from cyclonedx.handlers import (
    codebases_handler,
    codebase_handler,
)

from tests.conftest import create_harbor_table


@mock_dynamodb
def test_flow():

    """
    -> Test the creation, updating and deletion of a codebase.
    """

    db_client: HarborDBClient = HarborDBClient(
        dynamodb_resource=boto3.resource("dynamodb")
    )

    create_harbor_table(boto3.resource("dynamodb"))

    team_id: str = str(uuid.uuid4())
    project_id: str = str(uuid.uuid4())

    codebase_name: str = str(uuid.uuid4())
    language: str = "JAVA"
    build_tool: str = "MAVEN"

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name="Test Project Name",
                ),
            ],
        ),
        recurse=True,
    )

    # Create
    create_response: dict = create(
        team_id=team_id,
        project_id=project_id,
        name=codebase_name,
        language=language,
        build_tool=build_tool,
        handler=codebase_handler,
    )
    response_dict: dict = loads(create_response["body"])

    print(dumps(response_dict, indent=2))

    codebase_id: str = list(response_dict.keys()).pop()
    codebase_dict: dict = response_dict[codebase_id]
    assert codebase_name == codebase_dict[CodeBase.Fields.NAME]
    assert codebase_dict[CodeBase.Fields.NAME] == codebase_name
    assert codebase_dict[CodeBase.Fields.LANGUAGE] == language
    assert codebase_dict[CodeBase.Fields.BUILD_TOOL] == build_tool

    # Get Test 1
    get_response: dict = get(
        team_id=team_id,
        codebase_id=codebase_id,
        handler=codebase_handler,
    )
    response_dict = loads(get_response["body"])
    codebase_dict: dict = response_dict[codebase_id]
    assert codebase_name == codebase_dict[CodeBase.Fields.NAME]
    assert codebase_dict[CodeBase.Fields.NAME] == codebase_name
    assert codebase_dict[CodeBase.Fields.LANGUAGE] == language
    assert codebase_dict[CodeBase.Fields.BUILD_TOOL] == build_tool

    # Get Test 2
    get_response: dict = get_all(
        team_id=team_id,
        handler=codebases_handler,
    )
    response_dict = loads(get_response["body"])

    codebase_id: str = list(response_dict.keys()).pop()
    codebase_dict: dict = response_dict[codebase_id]
    assert codebase_name == codebase_dict[CodeBase.Fields.NAME]
    assert codebase_dict[CodeBase.Fields.NAME] == codebase_name
    assert codebase_dict[CodeBase.Fields.LANGUAGE] == language
    assert codebase_dict[CodeBase.Fields.BUILD_TOOL] == build_tool

    # Update
    new_codebase_name: str = str(uuid.uuid4())
    new_language: str = "KOTLIN"
    new_build_tool: str = "GRADLE"

    update(
        team_id=team_id,
        project_id=project_id,
        codebase_id=codebase_id,
        new_name=new_codebase_name,
        new_language=new_language,
        new_build_tool=new_build_tool,
        handler=codebase_handler,
    )

    test_codebase: CodeBase = db_client.get(
        CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
        )
    )

    assert new_codebase_name == test_codebase.name
    assert new_language == test_codebase.language
    assert new_build_tool == test_codebase.build_tool

    # Delete
    delete(
        team_id=team_id,
        codebase_id=codebase_id,
        handler=codebase_handler,
    )

    # Get Test (Should return nothing)
    try:
        get_response: dict = get(
            team_id=team_id,
            codebase_id=codebase_id,
            handler=codebase_handler,
        )
        print(get_response)
        pytest.fail()
    except DatabaseError:
        db_client.delete(Team(team_id=team_id))
        print("All clear.  Database is clean")


# pylint: disable=R0913
def create(
    team_id: str,
    project_id: str,
    name: str,
    language: str,
    build_tool: str,
    handler,
):

    """
    -> Create a codebase
    """

    event: dict = {
        "requestContext": {
            "http": {
                "method": "POST",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
            "projectId": project_id,
        },
        "body": dumps(
            {
                "name": name,
                "language": language,
                "buildTool": build_tool,
            }
        ),
    }

    return handler(event, {})


def get(team_id: str, codebase_id: str, handler):

    """
    -> Get a codebase
    """

    event: dict = {
        "pathParameters": {
            "codebase": codebase_id,
        },
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
    }

    return handler(event, {})


def get_all(team_id: str, handler):

    """
    -> Get all the codebases
    """

    event: dict = {
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
    }

    return handler(event, {})


# pylint: disable=R0913
def update(
    team_id: str,
    codebase_id: str,
    project_id: str,
    new_name: str,
    new_language: str,
    new_build_tool: str,
    handler,
):
    """
    -> Update a codebase's data
    """

    event: dict = {
        "pathParameters": {
            "codebase": codebase_id,
        },
        "requestContext": {
            "http": {
                "method": "PUT",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
            "projectId": project_id,
        },
        "body": dumps(
            {
                CodeBase.Fields.NAME: new_name,
                CodeBase.Fields.LANGUAGE: new_language,
                CodeBase.Fields.BUILD_TOOL: new_build_tool,
            }
        ),
    }

    return handler(event, {})


def delete(team_id: str, codebase_id: str, handler):

    """
    -> Delete a codebase
    """

    event: dict = {
        "pathParameters": {
            "codebase": codebase_id,
        },
        "requestContext": {
            "http": {
                "method": "DELETE",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
    }

    return handler(event, {})