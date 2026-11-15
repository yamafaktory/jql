| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.7 ± 0.2 |  |  |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.5 ± 0.5 | -8.8% |  |

| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.6 ± 0.1 |  |  |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.4 ± 0.4 | -8.2% |  |

| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.6 ± 0.1 |  |  |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.4 ± 0.4 | -9.2% |  |

| Command | Mean Wall Time [s] | Change | Factor |
|:---|---:|---:|:---|
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 1.041 ± 0.042 |  |  |
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 0.107 ± 0.004 | -89.7% | (9.7x faster) |

| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 921.4 ± 44.5 |  |  |
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 120.7 ± 1.4 | -86.9% | (7.6x faster) |

