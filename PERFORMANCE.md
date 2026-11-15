| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 1.8 ± 0.1 |  |  |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.6 ± 0.2 | -11.8% |  |

| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 1.7 ± 0.0 |  |  |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.6 ± 0.3 | -10.4% |  |

| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 1.7 ± 0.0 |  |  |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.6 ± 0.3 | -8.1% |  |

| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 613.4 ± 8.9 |  |  |
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 115.7 ± 1.9 | -81.1% | (5.3x faster) |

| Command | Mean Wall Time [ms] | Change | Factor |
|:---|---:|---:|:---|
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 537.7 ± 4.7 |  |  |
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 137.5 ± 2.5 | -74.4% | (3.9x faster) |

